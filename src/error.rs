use rocket::http::Status;
use rocket::response::Responder;
use rocket::{Request, Response, response};

use crate::basic_authorization::AuthenticationError;

/// Potential errors of the API.
#[derive(Debug)]
pub enum Error {
    /// The request was unauthorized.
    NotAuthorized(AuthenticationError),
    /// The requested resource was not found.
    NotFound,
}

impl From<AuthenticationError> for Error {
    fn from(err: AuthenticationError) -> Self {
        Self::NotAuthorized(err)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        match self {
            Self::NotAuthorized(error) => error.respond_to(request),
            Self::NotFound => Response::build().status(Status::NotFound).ok(),
        }
    }
}
