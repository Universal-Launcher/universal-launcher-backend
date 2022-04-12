use actix_csrf::{
    extractor::{CsrfCookieConfig, CsrfHeaderConfig},
    CsrfMiddleware,
};
use actix_web::http::header::HeaderName;
use actix_web::{
    error::{self, QueryPayloadError},
    http::Method,
    web, HttpRequest, HttpResponse,
};
use rand::prelude::StdRng;

use crate::utils::errors::ErrorHandling;

mod base;
mod modpack_versions;
mod modpacks;
mod user;

pub fn router(cfg: &mut web::ServiceConfig) {
    let csrf = CsrfMiddleware::<StdRng>::new()
        .cookie_name("universal-launcher-csrf")
        .http_only(false)
        .set_cookie(Method::GET, "/panel/");

    let csrf_cookie_config = CsrfCookieConfig::new("universal-launcher-csrf".to_string());
    let csrf_header_config = CsrfHeaderConfig::new(HeaderName::from_static("X-XSRF-TOKEN"));

    cfg.service(
        web::scope("/panel")
            .app_data(csrf_cookie_config)
            .app_data(csrf_header_config)
            .service(base::register())
            .service(user::register())
            .service(modpacks::register())
            .service(modpack_versions::register())
            .wrap(csrf),
    );

    cfg.app_data(web::JsonConfig::default().error_handler(json_error_handler));

    configure_errors(cfg);
}

fn configure_errors(cfg: &mut web::ServiceConfig) {
    // Query string default configuration
    cfg.app_data(
        web::QueryConfig::default().error_handler(|err: QueryPayloadError, _| {
            let resp = HttpResponse::BadRequest().json(ErrorHandling::new(err.to_string()));
            error::InternalError::from_response(err, resp).into()
        }),
    );
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let details = ErrorHandling::new(err.to_string());

    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(details),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(details)
        }
        _ => HttpResponse::BadRequest().json(details),
    };

    error::InternalError::from_response(err, resp).into()
}
