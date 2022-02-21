use crate::handlers::HttpAsyncResponse;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use url::Url;

const MS_AUTHORIZE_URL: &str = "https://login.live.com/oauth20_authorize.srf";

pub async fn redirect() -> HttpAsyncResponse {
    let client_id = std::env::var("MSA_CLIENT_ID").unwrap();
    let redirect_uri = std::env::var("CLIENT_REDIRECT_URI").unwrap();

    println!("{:?} / {:?}", client_id, redirect_uri);

    let mut u = Url::parse(MS_AUTHORIZE_URL).unwrap();
    u.query_pairs_mut()
        .append_pair("client_id", client_id.as_str());
    u.query_pairs_mut().append_pair("response_type", "code");
    u.query_pairs_mut()
        .append_pair("redirect_uri", redirect_uri.as_str());
    u.query_pairs_mut()
        .append_pair("scope", "XboxLive.signin offline_access");

    let response = HttpResponse::TemporaryRedirect()
        .insert_header(("Location", u.as_str()))
        .finish();
    Ok(response)
}

#[derive(Deserialize)]
pub struct MSAuthCode {
    pub code: String,
}

pub async fn fallback(q: web::Query<MSAuthCode>) -> HttpAsyncResponse {
    let q = q.into_inner();

    println!("MS Auth code: {}", q.code);
    Ok(HttpResponse::Ok().finish())
}
