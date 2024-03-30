use uuid::Uuid;
use actix_identity::Identity;
use actix_web::{cookie::Display, HttpMessage, HttpRequest, HttpResponse}; 
use tera::Context;

use crate::{lib::user::new_client, web::errors::RedHttpError};
use crate::lib::common::RedLogin;

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

pub async fn red_login(form: actix_web::web::Form<RedLogin>,
                        red_users: actix_web::web::Data<crate::RedUsers>,
                        request: HttpRequest) -> Result<HttpResponse, RedHttpError> {
    let user = new_client("ssh", form.into_inner()).map_err( |_e| RedHttpError::new("Could not login with the given information.") )?;
    let id = Uuid::new_v4();
    Identity::login(&request.extensions(), id.to_string()).map_err( |_e| RedHttpError::default_error())?;
    red_users.lock().map_err( |_e| RedHttpError::default_error() )?.insert( id.to_string(), user );
    Ok(redirect("/red"))
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

pub async fn home(templates: actix_web::web::Data<tera::Tera>, 
                  red_users: actix_web::web::Data<crate::RedUsers>,
                  identity: Option<Identity>) -> Result<HttpResponse, RedHttpError> {
    match identity {
        Some(id) => {
            let uuid_str = id.id().map_err( |_e| RedHttpError::new("Could not get the session id"))?;
            let files = red_users.lock().map_err( |_e| RedHttpError::default_error() )?
                .get_mut(&uuid_str).unwrap()
                .get_files().map_err( |_e| RedHttpError::default_error() )?;

            let host = red_users.lock().map_err( |_e| RedHttpError::default_error() )?
                .get(&uuid_str).unwrap().get_host();
            let user = red_users.lock().map_err( |_e| RedHttpError::default_error() )?
                .get(&uuid_str).unwrap().get_username();
            
            Ok(render_template("red/home.html", crate::context!({"identity": uuid_str, 
                "host": host, "user": user, "files": files}), templates))
        },
        _ => {
            Ok(redirect("/"))
        }
    }
}

pub async fn red_logout(identity: Option<Identity>,
                        red_users: actix_web::web::Data<crate::RedUsers>) -> Result<HttpResponse, RedHttpError> {
    match identity {
        Some(id) => {
            let uuid_str = id.id().unwrap();
            let _ = red_users.lock().map_err( |_e| RedHttpError::default_error() )?
                .remove(&uuid_str);
            id.logout();
        },
        _ => {},
    }
    Ok(redirect("/red"))
}


/*



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
            HttpResponse::Forbidden().finish()
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
            HttpResponse::Forbidden().finish()
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

fn get_user_uuid(identity: Option<Identity>) -> String {
    match identity {
        Some(id) => id.id().unwrap(),
        _ => return "".into()
    }
}

pub async fn new_file(target: actix_web::web::Json<Redfile>, 
                      identity: Option<Identity>, 
                      red_users: actix_web::web::Data<crate::RedUsers>) -> HttpResponse {
    let user_uuid = get_user_uuid(identity);
    if user_uuid.is_empty() {
        return HttpResponse::Forbidden().finish()
    }
    let filename = target.get_uuid();
    let file_uuid = red_users.lock().unwrap().get_mut(&user_uuid).unwrap().create_new_file(filename);
    let files = red_users.lock().unwrap().get_mut(&user_uuid).unwrap().execute_file();
    HttpResponse::Ok().json( crate::json_response!({"file_uuid": file_uuid, "files": files}) )
}

pub async fn save_file(target: actix_web::web::Json<Redfile>, 
                        identity: Option<Identity>, 
                        red_users: actix_web::web::Data<crate::RedUsers>) 
                        -> HttpResponse {
    let user_uuid = get_user_uuid(identity);
    if user_uuid.is_empty() {
        return HttpResponse::Forbidden().finish()
    }
    
    if target.get_file_content().is_none() {
        return HttpResponse::InternalServerError().finish()
    }
    
    let result = red_users.lock().unwrap().get_mut(&user_uuid).unwrap()
                        .save_file(target.get_uuid(), target.get_file_content().unwrap());
    HttpResponse::Ok().json( crate::json_response!({"message": result }) )
}

*/