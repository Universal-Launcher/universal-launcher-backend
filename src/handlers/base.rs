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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;

    #[actix_web::test]
    async fn handle_index() {
        let resp = index().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
