//Global imports
#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;

//Internal imports 
mod web;
mod lib;

#[launch]
fn rocket() -> _ {
    let mut instance = rocket::build();
    instance = web::web(instance);
    instance.attach(Template::fairing())
}
