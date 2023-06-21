//Global imports
use actix_web::{HttpServer, App};
use actix_web::middleware::Logger;
use tera::Tera;
use env_logger::Env;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::time::Duration;

//Internal imports 
mod web;
mod lib;

const ONE_MINUTE: Duration = Duration::minutes(1);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*::std::env::set_var("RUST_BACKTRACE", "1");*/

    env_logger::init_from_env(Env::default().default_filter_or("trace"));
    let private_key = actix_web::cookie::Key::generate();
    //let store = 

    HttpServer::new(move || {
        let tera = Tera::new("src/templates/**/*.html").unwrap();
        App::new()
            //login 
            .wrap(Logger::default())
            //session management 
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder( CookieSessionStore::default(), private_key.clone() )
                .cookie_name("red".to_owned())
                .cookie_secure(true)
                .session_lifecycle(PersistentSession::default().session_ttl(ONE_MINUTE))
                .build(),
            )
            .app_data(actix_web::web::Data::new(tera))
            .configure(web::get_configuration)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

