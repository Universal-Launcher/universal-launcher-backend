use actix_csrf::extractor::{Csrf, CsrfHeader};
use actix_session::Session;
use actix_web::{http, web, HttpResponse};
use actix_web_4_validator::Json;

use crate::database::db::DbPool;
use crate::database::models::user::{NewUser, UserLogin};
use crate::utils::errors::ErrorHandling;
use crate::{actions, utils};

use super::HttpAsyncResponse;

pub async fn register_user(
    new_user: Json<NewUser>,
    data: web::Data<utils::Data>,
    pool: web::Data<DbPool>,
) -> HttpAsyncResponse {
    let user_check = new_user.clone();
    let pool_check = pool.clone();

    let result =
        web::block(move || actions::users::check_if_exists(&user_check, &pool_check)).await?;

    if result.is_err() {
        let (status, message) = result.unwrap_err();
        return Ok(HttpResponse::build(status).json(ErrorHandling::new(message)));
    }

    if result.unwrap() {
        return Ok(HttpResponse::build(http::StatusCode::CONFLICT)
            .json(ErrorHandling::new("errors.auth.already_exists".to_string())));
    }

    let result = web::block(move || actions::users::create_user(&new_user, &data, &pool)).await?;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            let (status, message) = e;
            Ok(HttpResponse::build(status).json(ErrorHandling::new(message)))
        }
    }
}

pub async fn authenticate_user(
    _: Csrf<CsrfHeader>,
    user: Json<UserLogin>,
    data: web::Data<utils::Data>,
    pool: web::Data<DbPool>,
    session: Session,
) -> HttpAsyncResponse {
    let login_result =
        web::block(move || actions::users::login_user(&user.0, &data, &pool)).await?;

    if login_result.is_err() {
        let (status, message) = login_result.unwrap_err();
        return Ok(HttpResponse::build(status).json(ErrorHandling::new(message)));
    }

    let found_user = login_result.unwrap();

    let session_result = session.insert("user", found_user.id);

    println!("{:?}", session_result);
    if session_result.is_err() {
        println!("{:?}", session_result.unwrap_err());
        return Ok(HttpResponse::InternalServerError().finish());
    }

    Ok(HttpResponse::Ok().json(found_user))
}

pub async fn get_user(session: Session, pool: web::Data<DbPool>) -> HttpAsyncResponse {
    let val = session.get::<i32>("user")?;

    if val.is_none() {
        return Ok(HttpResponse::build(http::StatusCode::UNAUTHORIZED)
            .json(ErrorHandling::new("errors.auth.not_logged_in".to_string())));
    }

    let result = web::block(move || {
        let id = val.unwrap();
        actions::users::get_user(id, &pool)
    })
    .await?;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            let (status, message) = e;
            Ok(HttpResponse::build(status).json(ErrorHandling::new(message)))
        }
    }
}

pub async fn logout_user(session: Session) -> HttpAsyncResponse {
    let val = session.get::<i32>("user")?;

    if val.is_some() {
        session.remove("user");
    }

    Ok(HttpResponse::Ok().finish())
}
