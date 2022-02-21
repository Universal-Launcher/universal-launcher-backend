pub type HttpAsyncResponse = Result<actix_web::HttpResponse, actix_web::Error>;

pub mod base;
pub mod modpack_versions;
pub mod modpacks;
pub mod user;
