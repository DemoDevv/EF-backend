use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use futures_core::ready;
use futures_util::future::{LocalBoxFuture, TryFutureExt as _};

use actix_web::{dev::ServiceRequest, Error, FromRequest};

struct Extract<T> {
    req: Option<ServiceRequest>,
    fut: Option<LocalBoxFuture<'static, Result<T, Error>>>,
    _extractor: PhantomData<fn() -> T>,
}

impl<T> Extract<T> {
    pub fn new(req: ServiceRequest) -> Self {
        Extract {
            req: Some(req),
            fut: None,
            _extractor: PhantomData,
        }
    }
}

impl<T> Future for Extract<T>
where
    T: FromRequest,
    T::Future: 'static,
    T::Error: 'static,
{
    type Output = Result<(ServiceRequest, T), (Error, ServiceRequest)>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.fut.is_none() {
            let req = self.req.as_mut().expect("Extract future was polled twice!");
            let fut = req.extract::<T>().map_err(Into::into);
            self.fut = Some(Box::pin(fut));
        }

        let fut = self
            .fut
            .as_mut()
            .expect("Extraction future should be initialized at this point");

        let credentials = ready!(fut.as_mut().poll(ctx)).map_err(|err| {
            (
                err,
                // returning request allows a proper error response to be created
                self.req.take().expect("Extract future was polled twice!"),
            )
        })?;

        let req = self.req.take().expect("Extract future was polled twice!");
        Poll::Ready(Ok((req, credentials)))
    }
}
