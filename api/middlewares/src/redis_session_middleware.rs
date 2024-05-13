use std::rc::Rc;

use futures_util::future::{self, LocalBoxFuture};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};

use api_services::redis::{models::Session, RedisClient};

use futures_util::FutureExt;

pub struct RedisSessionMiddlewareFactory {
    redis_repository: Rc<RedisClient>,
}

impl RedisSessionMiddlewareFactory {
    // this middleware is used when you want to get the session form redis on your route.
    // You have to use the session extractor for get information from the session in your handlers.
    pub fn new(redis_repository: RedisClient) -> Self {
        RedisSessionMiddlewareFactory {
            redis_repository: Rc::new(redis_repository),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RedisSessionMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
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
    redis_repository: Rc<RedisClient>,
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RedisSessionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let redis_repository = self.redis_repository.clone();

        async move {
            println!("TestMiddleware: before call");

            // Use Extract for use extractor in middleware

            let temp_session_payload = Session {};
            // use redis_repository for get the session of the current user

            req.extensions_mut()
                .insert::<SessionPayload>(Rc::new(temp_session_payload));

            let res = srv.call(req).await?;

            Ok(res)
        }
        .boxed_local()
    }
}
