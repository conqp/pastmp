use argon2::{PasswordVerifier, password_hash};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::Responder;
use rocket::{Request, Response, response};

use crate::accounts::Accounts;

/// A user account.
#[derive(Clone, Debug)]
pub struct BasicAuthorization {
    user_name: String,
    password: String,
}

impl BasicAuthorization {
    /// Validate the password.
    pub fn validate(
        &self,
        accounts: &Accounts,
        verifier: &dyn PasswordVerifier,
    ) -> Result<(), AuthenticationError> {
        let Some(password_hash) = accounts.get(&self.user_name) else {
            return Err(AuthenticationError::NoSuchUser);
        };

        password_hash
            .password_hash()
            .verify_password(&[verifier], &self.password)
            .map_err(AuthenticationError::InvalidPassword)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AuthenticationError {
    NoSuchUser,
    InvalidPassword(password_hash::Error),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AuthenticationError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'o> {
        Response::build().status(Status::Unauthorized).ok()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuthorization {
    type Error = FromRequestError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(auth) = request.headers().get_one("Authorization") else {
            return Outcome::Error((Status::Unauthorized, FromRequestError::Missing));
        };

        let Some((typ, b64)) = auth.split_once(' ') else {
            return Outcome::Error((Status::Unauthorized, FromRequestError::Malformatted));
        };

        if typ.to_lowercase() != "basic" {
            return Outcome::Error((Status::Unauthorized, FromRequestError::NotBasic));
        }

        let Ok(auth) = BASE64_STANDARD.decode(b64) else {
            return Outcome::Error((Status::Unauthorized, FromRequestError::NotBase64));
        };

        let Ok(auth) = String::from_utf8(auth) else {
            return Outcome::Error((Status::Unauthorized, FromRequestError::NotUtf8));
        };

        let Some((user_name, password)) = auth.split_once(':') else {
            return Outcome::Error((
                Status::Unauthorized,
                FromRequestError::NotUserNameAndPassword,
            ));
        };

        Outcome::Success(Self {
            user_name: user_name.to_string(),
            password: password.to_string(),
        })
    }
}

#[derive(Debug)]
pub enum FromRequestError {
    Missing,
    Malformatted,
    NotBasic,
    NotBase64,
    NotUtf8,
    NotUserNameAndPassword,
}
