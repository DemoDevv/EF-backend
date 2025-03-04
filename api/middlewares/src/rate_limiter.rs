use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use api_caches::token_buckets::{TokenBucketsCache, TokenBucketsCacheRedis};
use api_errors::ServiceError;
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    sync::Arc,
};

pub struct RateLimiter;

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware { service }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extraire les données nécessaires de la requête
        // On peut obtenir soit l'Id utilisateur depuis le token JWT, sinon il faut utiliser l'IP
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or_default()
            .to_string();

        let user_id = req
            .extensions()
            .get::<i32>()
            .map(|el| el.to_string())
            .unwrap_or(ip);

        let bucket_cache = req
            .app_data::<web::Data<Arc<TokenBucketsCacheRedis>>>()
            .cloned();

        let http_method = req.method().clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            if let Some(bucket_cache) = bucket_cache {
                let exists = bucket_cache
                    .bucket_exists(&user_id)
                    .await
                    .map_err(ServiceError::from)?;

                if !exists {
                    // we create a new bucket for the user
                    bucket_cache
                        .create_bucket(&user_id)
                        .await
                        .map_err(ServiceError::from)?;
                }

                bucket_cache
                    .refill_bucket(&user_id)
                    .await
                    .map_err(ServiceError::from)?;

                // can consume tokens or return a rate limit exceeded error
                bucket_cache
                    .consume_tokens(&user_id, &http_method)
                    .await
                    .map_err(ServiceError::from)?;
            } else {
                // Handle the case when bucket_cache is None
                // For example, you can log an error or return a default value
                log::error!(
                    "TokenBucketsCacheRedis not found, make sure it is set in your app data"
                );
            }

            fut.await
        })
    }
}
