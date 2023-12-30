use std::future::{Ready, ready};

use actix_web::{dev::{Transform, ServiceRequest, Service, ServiceResponse, forward_ready}, Error};
use futures_util::future::LocalBoxFuture;

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Transform = AuthMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S, // représentation abstraite de la requete gerée par le middleware
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S:Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        todo!("s'occuper de la gestion du JWT")
    }
}