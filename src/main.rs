#[allow(unused_imports)]

extern crate actix_web;
extern crate diesel;
#[macro_use]
extern crate slog;

extern crate jsonwebtoken;

use actix_web::middleware::Logger;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web, App, HttpServer, http::header};
use slog::{o, PushFnValue};
use slog::Drain;
use slog::FnValue;
use slog_term::{CompactFormat, TermDecorator, PlainDecorator};
use actix_slog::StructuredLogger;
use slog_json::Json;
use chrono::{Local, SecondsFormat};

mod api;
mod orm;
mod vars;
mod jwt;
mod constants;
mod middlewares;
mod root_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");

    env_logger::init();

    let pool: Pool<ConnectionManager<PgConnection>> = orm::db_connection::get_establish_connection(vars::database_url());

    let root_logger= root_logger::build_logger_root();

    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new(vars::redis_url())
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .app_data(root_logger.clone())
            // limit the maximum amount of data that server will accept
            .app_data(web::JsonConfig::default().limit(4096))
            // enable logger
            .wrap(Logger::new("%a %{User-Agent}i"))
            // .wrap(
            //     StructuredLogger::new(root_logger.new(
            //         o!("log_type" => "access", "msg" => "app"))
            //     ),
            // )
            //.wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone()
           ))
           .wrap(IdentityMiddleware::default())
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(vars::secret_key().as_bytes())
            //         .name("auth-cookie")
            //         .path("/")
            //         .domain(vars::domain().as_str())
            //         .max_age(86400) // one day in seconds
            //         .secure(false), // this can only be true if you have https
            // ))
            //.wrap(middlewares::RequestHandler::RequestHandler::new(root_logger.clone()))

            // Enable sessions
            // .wrap(
            //     CookieSession::signed(&[0; 32])
            //         .domain(vars::domain_url().as_str())
            //         .name("auth2")
            //         .secure(false))
            // .wrap(
            //     Cors::default()
            //         .allowed_origin("*")
            //         .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            //         .max_age(3600)
            // )
            .service(api::default::index)
            .service(api::identification::login)
            .service(api::identification::logout)
            //.service(api::newsletter::newsletters)
            //.service(api::newsletter::newsletter)
            //.service(api::image::image)
    })
        .bind(format!("{}:{}", vars::domain(), vars::port()))?
        .run()
        .await
}

