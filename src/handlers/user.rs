use actix_csrf::extractor::{Csrf, CsrfCookie, CsrfCookieConfig, CsrfHeader};
use actix_web::{http, web, HttpRequest, HttpResponse};
use actix_web_4_validator::Json;

use crate::database::database::DbPool;
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
) -> HttpAsyncResponse {
    let result = web::block(move || actions::users::login_user(&user.0, &data, &pool)).await?;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            let (status, message) = e;
            Ok(HttpResponse::build(status).json(ErrorHandling::new(message)))
        }
    }
}

pub async fn get_user() -> HttpAsyncResponse {
    Ok(HttpResponse::Ok().finish())
}

pub async fn logout_user() -> HttpAsyncResponse {
    Ok(HttpResponse::Ok().finish())
}
