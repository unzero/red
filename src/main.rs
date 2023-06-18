//Global imports
use actix_web::{HttpServer, App};
use actix_web::middleware::Logger;
use tera::Tera;
use env_logger::Env;

//Internal imports 
mod web;
mod lib;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*::std::env::set_var("RUST_BACKTRACE", "1");*/

    env_logger::init_from_env(Env::default().default_filter_or("trace"));

    HttpServer::new(|| {
        let tera = Tera::new("src/templates/**/*.html").unwrap();
        App::new()
            .app_data(actix_web::web::Data::new(tera))
            .configure(web::get_configuration)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

