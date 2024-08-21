use std::fmt;


use actix_web::{error::ResponseError, HttpResponse};
use tera::Context;


use crate::web::common;


#[derive(Debug)]
pub enum RedHttpError {
    LoginError,
    LoginInternalError,
    SessionError,
    Default,
}


impl fmt::Display for RedHttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RedHttpError::LoginError  => write!(f, "Login Error"),
            RedHttpError::SessionError  => write!(f, "There was a problem with the session. Login again!"),
            _ => write!(f, "Something gone wrong!"),
        }
        
    }
}


impl ResponseError for RedHttpError {
    fn error_response(&self) -> HttpResponse {
        let templates = actix_web::web::Data::new(
            crate::web::utils::get_templates_route()
        );

        match self {
            RedHttpError::LoginError => common::render_template(
                "red/index.html", 
                crate::context!({"errors": vec!["login"]}), 
                templates
            ),
            RedHttpError::LoginInternalError => common::render_template(
                "red/index.html", 
                crate::context!({"errors": vec!["internal_login"]}), 
                templates
            ),
            _ => HttpResponse::InternalServerError().body("Something gone wrong, try again!"),
        }
    }

}
        