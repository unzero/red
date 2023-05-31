use rocket::response::Redirect;
use rocket::http::{Status};
use rocket::form::{Form, Context, FromForm};
use rocket_dyn_templates::{Template, context};

#[derive(FromForm, Debug)]
pub struct SshInformation<'v>{
    hostname: &'v str,
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
    (Status::Ok, Template::render("red/main", context!{hostname: form.hostname, password: form.password}))
}

