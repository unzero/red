use actix_web::HttpResponse; 
use tera::{Tera, Context};
use serde::{Deserialize, Serialize};

use crate::lib::connection::check_connection;
use crate::context;

#[derive(Debug, Deserialize, Serialize)]
pub struct SshInformation{
    host: String,
    user: String,
    password: String,
}

pub async fn red_submit(form: actix_web::web::Form<SshInformation>,
                        templates: actix_web::web::Data<tera::Tera>) -> HttpResponse {
    match check_connection(form.host.as_str(), 22, form.user.as_str(), form.password.as_str()) {
        true => HttpResponse::Ok().body("ok"),
        false => {
            render_template("red/index.html", crate::context!({"errors": {"login": "Wrong information"}}), templates)
        },
    }
}

pub async fn index(templates: actix_web::web::Data<tera::Tera>) -> HttpResponse {
    render_template("red/index.html", crate::context!({"errors": {}}), templates)
}

fn render_template(template_name: &str, 
                   context: &Context, 
                   templates: actix_web::web::Data<tera::Tera>) -> HttpResponse {

    let template = templates.render(template_name, context).expect("Error");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template)
}


