use std::{rc::Rc, sync::Arc};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};

use actix_web_httpauth::extractors::bearer::BearerAuth;
use api_services::redis::{errors::SessionError, models::Session, RedisClient, RedisRepository};

use crate::helpers::extract::Extract;

use futures_util::future::{self, LocalBoxFuture};
use futures_util::FutureExt;

pub struct RedisSessionMiddlewareFactory {
    redis_repository: Arc<RedisClient>,
}

impl RedisSessionMiddlewareFactory {
    // this middleware is used when you want to get the session form redis on your route.
    // You have to use the session extractor for get information from the session in your handlers.
    pub fn new(redis_repository: RedisClient) -> Self {
        RedisSessionMiddlewareFactory {
            redis_repository: Arc::new(redis_repository),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RedisSessionMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = RedisSessionMiddleware<S>;
    type InitError = ();
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(RedisSessionMiddleware {
            redis_repository: self.redis_repository.clone(),
            service: Rc::new(service),
        })
    }
}

pub type SessionPayload = Rc<Session>;

pub struct RedisSessionMiddleware<S> {
    redis_repository: Arc<RedisClient>,
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RedisSessionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let redis_repository = self.redis_repository.clone();

        async move {
            // get the token from the request
            let (req, token) = match Extract::<BearerAuth>::new(req).await {
                Ok(req) => req,
                Err((err, req)) => {
                    return Ok(req.error_response(err).map_into_right_body());
                }
            };

            // get the session from redis
            let session_opt = match redis_repository.get(token.token()).await {
                Ok(session) => session,
                Err(err) => {
                    return Ok(req
                        .error_response(SessionError::from(err))
                        .map_into_right_body());
                }
            };

            let session = match session_opt {
                Some(session) => session,
                None => {
                    return Ok(req
                        .error_response(SessionError::SessionNotFound)
                        .map_into_right_body());
                }
            };

            let session_deserialized: Session = serde_json::from_str(&session).unwrap();

            req.extensions_mut()
                .insert::<SessionPayload>(Rc::new(session_deserialized));

            srv.call(req).await.map(|res| res.map_into_left_body())
        }
        .boxed_local()
    }
}
