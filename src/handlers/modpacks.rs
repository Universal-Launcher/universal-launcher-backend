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
