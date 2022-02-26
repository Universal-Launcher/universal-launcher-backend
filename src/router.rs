use actix_web::error::QueryPayloadError;
use actix_web::{error, web, HttpRequest, HttpResponse};

use crate::utils::errors::ErrorHandling;

mod base;
mod modpack_versions;
mod modpacks;
mod user;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(base::register());
    cfg.service(modpacks::register());
    cfg.service(modpack_versions::register());
    cfg.service(user::register());

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
