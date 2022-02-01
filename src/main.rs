#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use std::sync::Arc;
use tokio;

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
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let port: String = std::env::var("PORT").unwrap_or(String::from("8080"));

    let db_pool = Arc::new(database::database::init_database().expect("Failed to init database"));

    println!("Booting server on port \"{}\"", port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(utils::Data {
                pool: Arc::clone(&db_pool),
            }))
            .configure(router)
            .wrap(middleware::Logger::default())
    })
    .workers(8)
    .bind(format!("0.0.0.0:{}", port))
    .expect("Couldn't bind to port {}", port)
    .run()
    .await
    .unwrap()
}
