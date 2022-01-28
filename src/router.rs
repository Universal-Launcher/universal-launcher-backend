use actix_web::error::QueryPayloadError;
use actix_web::{error, web, HttpResponse};
use serde::Serialize;

mod base;
mod modpacks;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(base::register());
    cfg.service(modpacks::register());

    configure_errors(cfg);
}

#[derive(Debug, Serialize)]
pub struct ErrorHandling {
    pub error: String,
}

fn configure_errors(cfg: &mut web::ServiceConfig) {
    // Query string default configuration
    cfg.app_data(
        web::QueryConfig::default().error_handler(|err: QueryPayloadError, _| {
            let resp = HttpResponse::BadRequest().json(ErrorHandling {
                error: err.to_string(),
            });
            error::InternalError::from_response(err, resp).into()
        }),
    );
}
