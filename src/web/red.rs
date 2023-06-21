use actix_web::{HttpResponse, HttpRequest, HttpMessage, web, http::StatusCode, Responder}; 
use tera::Context;
use serde::{Deserialize, Serialize};
use actix_identity::Identity;
use uuid::Uuid;

use crate::lib::connection::check_connection;

#[derive(Debug, Deserialize, Serialize)]
pub struct SshInformation{
    host: String,
    user: String,
    password: String,
}

pub async fn red_login(form: actix_web::web::Form<SshInformation>,
                        templates: actix_web::web::Data<tera::Tera>,
                        request: HttpRequest) -> HttpResponse {
    match check_connection(form.host.as_str(), 22, form.user.as_str(), form.password.as_str()) {
        true => {
            let id = Uuid::new_v4();
            Identity::login(&request.extensions(), id.to_string());
            redirect("/red")
        },
        false => {
            render_template("red/index.html", crate::context!({"errors": {"login": "Wrong information"}}), templates)
        },
    }
}

pub async fn home(templates: actix_web::web::Data<tera::Tera>, 
                  identity: Option<Identity>) -> HttpResponse {
    match identity {
        Some(id) => {
            render_template("red/home.html", crate::context!({"identity": id.id().unwrap()}), templates)
        },
        _ => {
            redirect("/")
        }
    }
}

pub async fn index(templates: actix_web::web::Data<tera::Tera>,
                   identity: Option<Identity>) -> HttpResponse {
    match identity {
        Some(id) => {
            redirect("/red")
        },
        _ => {
            render_template("red/index.html", crate::context!({"errors": {}}), templates)
        }
    }
}

fn render_template(template_name: &str, 
                   context: &Context, 
                   templates: actix_web::web::Data<tera::Tera>) -> HttpResponse {

    let template = templates.render(template_name, context).expect("Error");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template)
}

fn redirect(location: &str) -> HttpResponse{
    HttpResponse::Found()
                .insert_header(( actix_web::http::header::LOCATION, location, )).body("ok")
}

