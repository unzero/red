mod common;
mod errors;
pub mod red;
mod session;
pub mod utils;

//within this file I put all logic for web application
use actix_web::web;

//each module is an application
use crate::web::red::*;
pub use common::not_found;

pub fn get_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index))
            .default_service(actix_web::web::route().to(not_found)),
    );

    cfg.service(
        web::resource("/red")
            .route(web::post().to(red_login))
            .route(web::get().to(home))
            .default_service(actix_web::web::route().to(not_found)),
    );

    cfg.service(
        web::resource("/logout")
            .route(web::post().to(red_logout))
            .default_service(actix_web::web::route().to(not_found)),
    );

    cfg.service(
        web::resource("/open_file")
            .route(web::post().to(open_file))
            .default_service(actix_web::web::route().to(not_found)),
    );

    cfg.service(
        web::resource("/change_directory")
            .route(web::post().to(change_directory))
            .default_service(actix_web::web::route().to(not_found)),
    );

    cfg.service(
        web::resource("/new_file")
            .route(web::post().to(new_file))
            .default_service(actix_web::web::route().to(not_found)),
    );

    cfg.service(
        web::resource("/save_file")
            .route(web::post().to(save_file))
            .default_service(actix_web::web::route().to(not_found)),
    );
}
