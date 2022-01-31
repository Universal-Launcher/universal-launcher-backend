use actix_web::{web, HttpResponse};

extern crate diesel;
use crate::database::models::*;
use crate::database::schema::modpacks::dsl::*;
use diesel::prelude::*;

use crate::utils;

pub async fn index(data: web::Data<utils::Data>) -> HttpResponse {
    let d = data.as_ref();
    let conn = &mut d
        .pool
        .get()
        .expect("Failed to access to database connection instance");

    let result = modpacks
        .load::<Modpack>(conn)
        .expect("Error loading modpacks");

    HttpResponse::Ok().body(format!("There is {} modpacks in registered", result.len()))
}

pub async fn create(data: web::Data<utils::Data>, item: web::Json<NewModpack>) -> HttpResponse {
    use crate::database::schema::modpacks;
    let d = data.as_ref();

    let conn = &mut d
        .pool
        .get()
        .expect("Failed to access to database connection instance");

    let modpack = NewModpack {
        name: item.name.clone(),
        description: item.description.clone(),
    };

    diesel::insert_into(modpacks::table)
        .values(&modpack)
        .execute(conn)
        .expect("Error saving new modpack");
    /*.get_result::<Modpack>(conn)
    .expect("Error saving new modpack");*/

    HttpResponse::Ok().body("")
}

pub async fn show() -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn update() -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn delete() -> HttpResponse {
    HttpResponse::Ok().body("")
}
