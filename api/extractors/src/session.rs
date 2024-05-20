use std::{
    future::{ready, Ready},
    ops::Deref,
};

use actix_web::{Error, FromRequest, HttpMessage};

use api_services::redis::errors::SessionError;
use api_services::redis::models::Session as SessionPayload;

pub struct Session(SessionPayload);

impl FromRequest for Session {
    type Error = Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let session_extracted = req.extensions().get::<SessionPayload>().cloned();

        let session = match session_extracted {
            Some(session_payload) => Ok(Session(session_payload)),
            None => Err(SessionError::SessionExtractionError.into()),
        };

        ready(session)
    }
}

impl Deref for Session {
    type Target = SessionPayload;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
