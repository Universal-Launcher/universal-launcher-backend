use actix_web::{web, HttpResponse};
extern crate diesel;
use crate::database::models::modpacks::*;
use crate::{actions, handlers::HttpAsyncResponse, utils};

#[derive(serde::Serialize)]
struct ModpackList {
    modpacks: Vec<Modpack>,
}

pub async fn index(data: web::Data<utils::Data>) -> HttpAsyncResponse {
    let result = web::block(move || actions::modpacks::find_all_modpacks(&data.as_ref().pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(ModpackList { modpacks: result }))
}

pub async fn create(
    data: web::Data<utils::Data>,
    item: web::Json<NewModpack>,
) -> HttpAsyncResponse {
    let modpack = NewModpack {
        name: item.name.clone(),
        description: item.description.clone(),
    };

    web::block(move || actions::modpacks::insert_modpack(modpack, &data.as_ref().pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(""))
}

pub async fn show(data: web::Data<utils::Data>, path: web::Path<(i32,)>) -> HttpAsyncResponse {
    let (m_id,) = path.into_inner();

    let modpack =
        web::block(move || actions::modpacks::find_modpack_by_id(m_id, &data.as_ref().pool))
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(modpack) = modpack {
        Ok(HttpResponse::Ok().json(modpack))
    } else {
        let res = HttpResponse::NotFound().body("");
        Ok(res)
    }
}

pub async fn update(
    data: web::Data<utils::Data>,
    path: web::Path<(i32,)>,
    item: web::Json<UpdateModpack>,
) -> HttpAsyncResponse {
    let (m_id,) = path.into_inner();

    web::block(move || actions::modpacks::update_by_id(m_id, &data.as_ref().pool, item.0))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(""))
}

pub async fn delete(data: web::Data<utils::Data>, path: web::Path<(i32,)>) -> HttpAsyncResponse {
    let (m_id,) = path.into_inner();

    web::block(move || actions::modpacks::delete_by_id(m_id, &data.as_ref().pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(""))
}
