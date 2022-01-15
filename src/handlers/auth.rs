use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};

use crate::api_calls;

#[derive(Serialize)]
struct Info {
    status: String,
}

pub async fn index() -> Result<HttpResponse, Error> {
    let info = Info {
        status: String::from("ok"),
    };
    let resp = HttpResponse::Ok().content_type("text/plain").json(info);

    Ok(resp)
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct AuthCode {
    #[serde_as(as = "NoneAsEmptyString")]
    code: Option<String>,
}

pub async fn token(web::Query(qs): web::Query<AuthCode>) -> Result<HttpResponse, Error> {
    let code: Option<String> = qs.code;
    let res = match code {
        Some(c) => {
            let result = api_calls::get_authorization_token_from_code(c).await;
            match result {
                Ok(res) => HttpResponse::Ok().body(res),
                Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
            }
        }
        None => HttpResponse::BadRequest().finish(),
    };

    Ok(res)
}
