use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;

pub struct TestMiddlewareFactory;

impl TestMiddlewareFactory {
    pub fn new() -> Self {
        TestMiddlewareFactory
    }
}

impl<S, B> Transform<S, ServiceRequest> for TestMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TestMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TestMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct TestMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for TestMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Clone the Rc pointers so we can move them into the async block.
        let srv = self.service.clone();

        async move {
            println!("TestMiddleware: before call");

            let res = srv.call(req).await?;

            Ok(res)
        }
        .boxed_local()
    }
}
