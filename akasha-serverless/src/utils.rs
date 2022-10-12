use core::fmt;

use cfg_if::cfg_if;
use ed25519_dalek::{SignatureError, ed25519};
use hex::FromHexError;
use thiserror::Error;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

#[derive(Debug)]
pub enum HttpStatusCode {
    BadRequest = 400,
    Unauthorized = 401,
    InternalServerError = 500,
}

#[derive(Debug)]
pub enum Error {
    EnvironmentVariableNotFound(String),
    HeaderNotFound(String),
    JsonFailed(serde_json::Error),
    PayloadError(String),
    VerificationFailed(VerificationError),
    InteractionFailed(InteractionError)
}

#[derive(Debug)]
pub enum InteractionError {
    CommandNotFound(String),
    CloudflareError(worker::Error),
    DiscordError(String),
    Error(String),
}

#[derive(Debug, thiserror::Error)]
pub enum VerificationError {
    #[error("Could not parse hex")]
    ParseError(#[from] FromHexError),
    #[error("Invalid Public Key")]
    InvalidKey(#[from] SignatureError),
    #[error("Invalid Signature")]
    InvalidSignature(ed25519::Error),
}

#[derive(Debug)]
pub struct HttpError {
    pub status: HttpStatusCode,
    reason: Error
}

impl From<Error> for HttpError {
    fn from(reason: Error) -> HttpError {
        let status = match &reason {
            Error::HeaderNotFound(_) => HttpStatusCode::BadRequest,
            Error::JsonFailed(_) => HttpStatusCode::BadRequest,
            Error::PayloadError(_) => HttpStatusCode::BadRequest,
            Error::VerificationFailed(_) => HttpStatusCode::Unauthorized,
            _ => HttpStatusCode::InternalServerError,
        };
        HttpError {
            status,
            reason, 
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error occured: {:?}", self.reason)
    }
}
