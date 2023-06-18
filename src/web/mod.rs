/*//within this file I put all logic for web application
use rocket::{Rocket, Build};

//each module is an application 
pub mod red;
use crate::web::red::{red_index, index, red_submit};

//Include here all the paths to red application
pub fn web(instance: Rocket<Build>) -> Rocket<Build> {
    instance
        .mount("/", routes![index])
        .mount("/", routes![red_index])
        .mount("/", routes![red_submit])
}*/

//within this file I put all logic for web application
use actix_web::web;

//each module is an application 
pub mod red;
pub mod utils;
use crate::web::red::{index, red_submit};

pub fn get_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index)),
    );
    cfg.service(
        web::resource("/red")
            .route(web::post().to(red_submit)),
    );
}
