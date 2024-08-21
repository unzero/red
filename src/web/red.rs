use actix_web::Result;
use uuid::Uuid;
use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse}; 
use tera::Context;

use super::session;
use super::utils;
use crate::{lib::user::new_client, web::errors::RedHttpError};
use crate::lib::common::RedLogin;
use crate::lib::files::Redfile;
use crate::lib::files;
use crate::web::common;

pub async fn index(templates: actix_web::web::Data<tera::Tera>,
                   identity: Option<Identity>) -> HttpResponse {
    match identity {
        Some(_) => {
            common::redirect("/red")
        },
        _ => {
            common::render_template("red/index.html", crate::context!({"errors": {}}), templates)
        }
    }
}

pub async fn red_login(form: actix_web::web::Form<RedLogin>,
                        red_users: actix_web::web::Data<crate::RedUsers>,
                        request: HttpRequest) -> Result<HttpResponse, RedHttpError> {
    let user = new_client("ssh", form.into_inner()).map_err( |_e| RedHttpError::LoginError )?;
    let id = Uuid::new_v4();
    let _ = Identity::login(&request.extensions(), id.to_string()).map_err( |_e| RedHttpError::LoginInternalError );
    red_users.lock().map_err( |_e| RedHttpError::LoginInternalError )?.insert( id.to_string(), user );
    Ok( common::redirect("/red") )
}

pub async fn home(templates: actix_web::web::Data<tera::Tera>, 
                  red_users: actix_web::web::Data<crate::RedUsers>,
                  identity: Option<Identity>) -> Result<HttpResponse, RedHttpError> {

    let uuid_str = session::validate_session(identity)?;

    let files = red_users.lock().map_err( |_e| RedHttpError::Default )?
        .get_mut(&uuid_str).ok_or( RedHttpError::Default )?
        .get_files().map_err( |_e| RedHttpError::Default )?;

    let host = red_users.lock().map_err( |_e| RedHttpError::Default )?
        .get(&uuid_str).ok_or( RedHttpError::Default )?
        .get_host();

    let user = red_users.lock().map_err( |_e| RedHttpError::Default )?
        .get(&uuid_str).ok_or( RedHttpError::Default )?
        .get_username();
            
    Ok( common::render_template(
            "red/home.html", 
            crate::context!(
                {
                    "identity": uuid_str, 
                    "host": host, 
                    "user": user, 
                    "files": files
                }
            ), 
            templates
        )
    )
        
}


pub async fn red_logout(identity: Option<Identity>,
                        red_users: actix_web::web::Data<crate::RedUsers>) -> Result<HttpResponse, RedHttpError> {

    let id = identity.ok_or( RedHttpError::SessionError )?;
    let uuid_str = id.id().map_err( |_e| RedHttpError::SessionError )?;      
    let _ = red_users.lock().map_err( |_e| RedHttpError::Default )?
        .remove(&uuid_str);
    id.logout();
    Ok( common::redirect("/") )
}

pub async fn open_file(target: actix_web::web::Json<Redfile>,
                       identity: Option<Identity>, 
                       red_users: actix_web::web::Data<crate::RedUsers>) -> Result<HttpResponse, RedHttpError> {
    let uuid_str = session::validate_session(identity)?;
    let filename = red_users.lock().unwrap().get_mut(&uuid_str).unwrap()
        .query_file_uuid(target.get_uuid()).map_err(|_e| RedHttpError::Default )?;
    let file_content = red_users.lock().unwrap().get_mut(&uuid_str).unwrap()
        .read_file_content(target.get_uuid()).map_err(|_e| RedHttpError::Default )?;
    let file_type = files::get_file_type(filename.clone());
    Ok(HttpResponse::Ok().json( crate::json_response!(
        {
            "file-content": file_content, 
            "file-type": file_type,
            "filename": filename,
            "file-uuid": target.get_uuid()
        }

    )))
}

pub async fn change_directory(target: actix_web::web::Json<Redfile>,
                       identity: Option<Identity>, 
                       red_users: actix_web::web::Data<crate::RedUsers>) -> Result<HttpResponse, RedHttpError> {
    let uuid_str = session::validate_session(identity)?;
    let files = red_users.lock().unwrap().get_mut(&uuid_str).unwrap()
        .change_directory(target.get_uuid()).map_err(|_e| RedHttpError::Default )?;
    Ok(HttpResponse::Ok().json( crate::json_response!({"files": files})))
}

pub async fn new_file(target: actix_web::web::Json<Redfile>, 
                      identity: Option<Identity>, 
                      red_users: actix_web::web::Data<crate::RedUsers>) -> Result<HttpResponse, RedHttpError> {
    let user_uuid = session::validate_session(identity)?;
    let filename = target.get_uuid();
    let file_uuid = red_users.lock().unwrap().get_mut(&user_uuid).unwrap()
        .create_new_file(filename).map_err(|_e| RedHttpError::Default )?;
    let files = red_users.lock().unwrap().get_mut(&user_uuid).unwrap()
        .get_files().map_err(|_e| RedHttpError::Default )?;
    Ok(HttpResponse::Ok().json( crate::json_response!({"file_uuid": file_uuid, "files": files})) )
}

pub async fn save_file(target: actix_web::web::Json<Redfile>, 
                        identity: Option<Identity>, 
                        red_users: actix_web::web::Data<crate::RedUsers>) 
                        -> Result<HttpResponse, RedHttpError> {
    let user_uuid = session::validate_session(identity)?;
    let content = target.get_file_content().ok_or_else(|| RedHttpError::Default )?;
    let result = red_users.lock().unwrap().get_mut(&user_uuid).unwrap()
                        .save_file(target.get_uuid(), content).map_err( |_e| RedHttpError::Default )?;
    Ok(HttpResponse::Ok().json( crate::json_response!({"message": result }) ) )
}
