//Global imports
use actix_web::{HttpServer, App};
use actix_web::middleware::Logger;
use env_logger::Env;
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::time::Duration;
use std::{sync::{Arc, Mutex}, collections::HashMap};

//Internal imports 
mod web;
mod lib;

use crate::lib::common::RedUsers;
use crate::web::not_found;

const COOKIE_LIFETIME: Duration = Duration::minutes(5);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* debug flag */
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init_from_env(Env::default().default_filter_or("trace"));
    let private_key = actix_web::cookie::Key::generate();
    let red_users: RedUsers = Arc::new(Mutex::new(HashMap::new()));
    let users_map_state = actix_web::web::Data::new(red_users);

    HttpServer::new( move || {
        let tera = crate::web::utils::get_templates_route();
        App::new()
            //login 
            .wrap(Logger::default())
            //session management 
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder( CookieSessionStore::default(), private_key.clone() )
                .cookie_name("red".to_owned())
                .cookie_secure(false)
                .session_lifecycle(PersistentSession::default().session_ttl(COOKIE_LIFETIME))
                .build(),
            )
            .app_data(actix_web::web::Data::new(tera))
            .app_data(users_map_state.clone())
            .configure(web::get_configuration)
            .service(actix_files::Files::new("/static", "./static/").show_files_listing())
            .default_service( actix_web::web::route().to( not_found ) )
    })
    .bind(("192.168.230.130", 8080))?
    .run()
    .await
}
