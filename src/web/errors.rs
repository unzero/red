use std::fmt;

use actix_web::error::ResponseError;

#[derive(Debug)]
pub struct RedHttpError {
    error: String
}

impl fmt::Display for RedHttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl RedHttpError {
    pub fn new(e: &str) -> Self {
        Self { error: e.into() }
    }

    pub fn default_error() -> Self {
        Self { error: "Something gone wrong, try again".into() }
    }

    pub fn session_error() -> Self {
        Self { error: "You don't have the right permisions for this.".into() }
    }
}

impl ResponseError for RedHttpError {}
