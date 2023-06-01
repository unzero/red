use rocket::response::Redirect;
use rocket::http::{Status};
use rocket::form::{Form, Context, FromForm};
use rocket_dyn_templates::{Template, context};

use crate::lib::connection::check_connection;

#[derive(FromForm, Debug)]
pub struct SshInformation<'v>{
    host: &'v str,
    user: &'v str,
    password: &'v str,
}

#[get("/")]
pub fn index() -> Redirect{ 
    Redirect::to(uri!("/", red_index()))
}

#[get("/red")]
pub fn red_index() -> Template {  
    Template::render("red/index", &Context::default())
}

#[post("/red", data="<form>")]
pub fn red_submit<'r>(form: Form<SshInformation<'_>>) -> (Status, Template){
    match check_connection(form.host, 22, form.user, form.password) {
        true => (Status::Ok, Template::render("red/main", context!{host: form.host, user: form.user, password: form.password})),
        false => (Status::Unauthorized, Template::render("red/index", 
                    context!{
                        errors: context!{login: "Wrong Login information!"}})),
    }
}

