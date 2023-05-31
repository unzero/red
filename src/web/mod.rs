//within this file I put all logic for web application
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
}

