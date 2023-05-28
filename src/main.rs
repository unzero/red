//Global imports
#[macro_use] extern crate rocket;

//Internal imports 
mod web;

#[launch]
fn rocket() -> _ {
    let mut instance = rocket::build();
    instance = web::web(instance);
    instance
}
