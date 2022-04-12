use actix_web::{web, HttpResponse};
extern crate diesel;
use crate::database::db::DbPool;
use crate::database::models::modpacks::*;
use crate::{actions, handlers::HttpAsyncResponse};

#[derive(serde::Serialize)]
struct ModpackList {
    modpacks: Vec<Modpack>,
}

pub async fn index(pool: web::Data<DbPool>) -> HttpAsyncResponse {
    let result = web::block(move || actions::modpacks::find_all_modpacks(&pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(ModpackList { modpacks: result }))
}

pub async fn create(
    new_modpack: web::Json<NewModpack>,
    pool: web::Data<DbPool>,
) -> HttpAsyncResponse {
    web::block(move || actions::modpacks::insert_modpack(&new_modpack, &pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn show(path: web::Path<(i32,)>, pool: web::Data<DbPool>) -> HttpAsyncResponse {
    let (m_id,) = path.into_inner();

    let modpack = web::block(move || actions::modpacks::find_modpack_by_id(m_id, &pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(modpack) = modpack {
        Ok(HttpResponse::Ok().json(modpack))
    } else {
        let res = HttpResponse::NotFound().finish();
        Ok(res)
    }
}

pub async fn update(
    path: web::Path<(i32,)>,
    item: web::Json<UpdateModpack>,
    pool: web::Data<DbPool>,
) -> HttpAsyncResponse {
    let (m_id,) = path.into_inner();

    web::block(move || actions::modpacks::update_by_id(m_id, &pool, item.0))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete(path: web::Path<(i32,)>, pool: web::Data<DbPool>) -> HttpAsyncResponse {
    let (m_id,) = path.into_inner();

    web::block(move || actions::modpacks::delete_by_id(m_id, &pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}
