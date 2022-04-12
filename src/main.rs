#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web, App, HttpResponse, HttpServer};

mod actions;
mod api_calls;
mod handlers;
mod router;
mod utils;

mod database;
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

    let port: String = std::env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let redis_connection_string = std::env::var("REDIS_URL").expect("REDIS_URL not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not set");

    let store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();

    println!("Booting server on port \"{}\"", port);
    HttpServer::new(move || {
        let cors_origin = std::env::var("CORS_ORIGIN").unwrap_or_else(|_| "".to_string());
        let pool = database::db::init_database();

        let mut cors = Cors::default()
            .allow_any_method()
            .supports_credentials()
            .max_age(3600);

        if cors_origin.is_empty() {
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
            .wrap(SessionMiddleware::new(
                store.clone(),
                Key::from(app_secret.as_bytes()),
            ))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .default_service(web::to(HttpResponse::Ok))
    })
    .workers(8)
    .bind(format!("0.0.0.0:{}", port))
    .unwrap_or_else(|_| panic!("Couldn't bind to port {}", port))
    .run()
    .await
    .unwrap()
}
