use actix_web::{middleware, App, HttpServer};

mod api_calls;
mod handlers;
mod router;
use router::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    println!("Booting server");

    HttpServer::new(|| {
        App::new()
            .configure(router)
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
