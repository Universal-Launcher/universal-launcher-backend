use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
struct Info {
    status: String,
}

pub async fn index() -> HttpResponse {
    let info = Info {
        status: String::from("ok"),
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(info)
}
