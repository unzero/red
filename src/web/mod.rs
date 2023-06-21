//within this file I put all logic for web application
use actix_web::web;

//each module is an application 
pub mod red;
pub mod utils;
use crate::web::red::{index, red_login, home};

pub fn get_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index)),
    );
    cfg.service(
        web::resource("/red")
            .route(web::post().to(red_login))
            .route(web::get().to(home)),
    );
}
