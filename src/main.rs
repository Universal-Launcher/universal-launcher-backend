#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_csrf::{extractor::CsrfCookieConfig, CsrfMiddleware};
use actix_web::{http::Method, middleware, web, App, HttpServer};
use rand::prelude::StdRng;
use tokio;

mod actions;
mod api_calls;
mod handlers;
mod router;
mod utils;

mod database;
use dotenv;
use router::router;

fn main() {
    actix_web::rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(8)
            .thread_name("main-tokio")
            .build()
            .unwrap()
    })
    .block_on(async_main());
}

async fn async_main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let port: String = std::env::var("PORT").unwrap_or(String::from("8080"));

    println!("Booting server on port \"{}\"", port);
    HttpServer::new(move || {
        let cors_origin = std::env::var("CORS_ORIGIN").unwrap_or("".to_string());
        let pool = database::database::init_database();

        let mut cors = Cors::default()
            .allow_any_method()
            .supports_credentials()
            .max_age(3600);

        if cors_origin.len() > 0 {
            cors = cors.allowed_origin(cors_origin.as_str());
        } else {
            println!("CORS_ORIGIN not set, CORS disabled");
            cors = cors.allow_any_origin();
        }

        let data = utils::load_data().expect("Failed to load data");

        App::new()
            //.app_data(web::Data::new(csrf_cookie_config))
            .app_data(web::Data::new(data))
            .app_data(web::Data::new(pool))
            .configure(router)
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
    })
    .workers(8)
    .bind(format!("0.0.0.0:{}", port))
    .expect(format!("Couldn't bind to port {}", port).as_str())
    .run()
    .await
    .unwrap()
}
