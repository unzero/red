use actix_web::{HttpResponse, HttpRequest, HttpMessage}; 
use tera::Context;
use serde::{Deserialize, Serialize};
use actix_identity::Identity;
use uuid::Uuid;

use crate::lib::connection::{check_connection, SshInformation};
use crate::lib::{clients::RedUser, files::Redfile};

#[derive(Debug, Deserialize, Serialize)]
pub struct RedLogin{
    host: String,
    user: String,
    password: String,
}

pub async fn red_login(form: actix_web::web::Form<RedLogin>,
                        templates: actix_web::web::Data<tera::Tera>,
                        red_users: actix_web::web::Data<crate::RedUsers>,
                        request: HttpRequest) -> HttpResponse {
    let ssh_info = SshInformation::new(form.host.clone(), 22, form.user.clone(), form.password.clone());
    match check_connection(ssh_info) {
        true => {
            let id = Uuid::new_v4();
            let _ = Identity::login(&request.extensions(), id.to_string());
            //TODO: hash the password to protect it 
            red_users.lock().unwrap().insert( id.to_string(), RedUser::new(
                        form.host.clone(), 
                        form.user.clone(), 
                        form.password.clone()) 
                    );
            redirect("/red")
        },
        false => {
            render_template("red/index.html", crate::context!({"errors": {"login": "Wrong information"}}), templates)
        },
    }
}

pub async fn home(templates: actix_web::web::Data<tera::Tera>, 
                  red_users: actix_web::web::Data<crate::RedUsers>,
                  identity: Option<Identity>) -> HttpResponse {
    match identity {
        Some(id) => {
            let uuid_str = id.id().unwrap();
            let files = red_users.lock().unwrap().get_mut(&uuid_str).unwrap().execute_file();
            let host = red_users.lock().unwrap().get(&uuid_str).unwrap().host.clone();
            let user = red_users.lock().unwrap().get(&uuid_str).unwrap().user.clone();
            render_template("red/home.html", crate::context!({"identity": id.id().unwrap(), 
                "host": host, "user": user, "files": files}), templates)
        },
        _ => {
            redirect("/")
        }
    }
}

pub async fn index(templates: actix_web::web::Data<tera::Tera>,
                   identity: Option<Identity>) -> HttpResponse {
    match identity {
        Some(_) => {
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

pub async fn red_logout(identity: Option<Identity>,
                        red_users: actix_web::web::Data<crate::RedUsers>) -> HttpResponse {
    match identity {
        Some(id) => {
            let uuid_str = id.id().unwrap();
            let red_user = red_users.lock().unwrap().remove(&uuid_str);
            id.logout();
        },
        _ => {},
    }
    redirect("/red")
}

pub async fn open_file(target: actix_web::web::Json<Redfile>,
                       identity: Option<Identity>, 
                       red_users: actix_web::web::Data<crate::RedUsers>) -> HttpResponse {
    match identity { 
        Some(id) => {
            let uuid_str = id.id().unwrap();
            let filename = red_users.lock().unwrap().get_mut(&uuid_str).unwrap().query_file_uuid(target.get_uuid());
            let file_content = red_users.lock().unwrap().get_mut(&uuid_str).unwrap().read_file_content(target.get_uuid());
            let file_type = get_file_type(filename.clone());
            HttpResponse::Ok().json( crate::json_response!(
                    {
                        "file-content": file_content, 
                        "file-type": file_type,
                        "filename": filename,
                        "file-uuid": target.get_uuid()
                    }

                ))
        },
        _ => {
            redirect("/")
        },
    }
    
}

pub async fn change_directory(target: actix_web::web::Json<Redfile>,
                       identity: Option<Identity>, 
                       red_users: actix_web::web::Data<crate::RedUsers>) -> HttpResponse {
    match identity { 
        Some(id) => {
            let uuid_str = id.id().unwrap();
            let files = red_users.lock().unwrap().get_mut(&uuid_str).unwrap().change_directory(target.get_uuid());
            HttpResponse::Ok().json( crate::json_response!({"files": files}) )
        },
        _ => {
            redirect("/")
        },
    }
    
}

fn get_file_type(filename: String) -> String {
    let ext = std::path::Path::new(&filename).
                extension().and_then(std::ffi::OsStr::to_str).unwrap_or("text");
    match ext { 
        "py" => String::from("python"),
        "rs" => String::from("rust"),
        "js" => String::from("javascript"),
        _ => String::from(ext)
    }
}
