pub mod red;
pub mod utils;
mod errors;

//within this file I put all logic for web application
use actix_web::web;

//each module is an application 
use crate::web::red::*;

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

    cfg.service(
        web::resource("/logout")
            .route(web::post().to(red_logout)),
    );

    
    cfg.service(
        web::resource("/open_file")
            .route(web::post().to(open_file)),
    );

    /*
    cfg.service(
        web::resource("/change_directory")
            .route(web::post().to(change_directory)),
    );

    cfg.service(
        web::resource("/new_file")
            .route(web::post().to(new_file)),
    );

    cfg.service(
        web::resource("/save_file")
            .route(web::post().to(save_file)),
    );*/

}


