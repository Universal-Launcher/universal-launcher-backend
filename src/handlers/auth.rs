use actix_web::{Error, HttpResponse};

pub async fn index() -> Result<HttpResponse, Error> {
    let resp = HttpResponse::Ok().content_type("text/plain").body("salut");

    Ok(resp)
}
