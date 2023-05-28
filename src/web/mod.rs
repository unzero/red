//within this file I put all logic for web application
use rocket::{Rocket, Build};

//each module is an application 
pub mod red;
use crate::web::red::red as red_main;

//Include here all the paths to red application
pub fn web(instance: Rocket<Build>) -> Rocket<Build> {
   instance.mount("/", routes![red_main])
}

