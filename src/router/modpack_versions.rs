use actix_web::{web, Scope};

pub fn register() -> Scope {
    web::scope("/modpack/{m_id}/versions")
}
