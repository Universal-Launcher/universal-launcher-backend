use actix_web::{web, HttpResponse};
use actix_web_4_validator::Json;

use crate::database::models::user::{NewUser, UserLogin};
use crate::utils::errors::ErrorHandling;
use crate::{actions, utils};

use super::HttpAsyncResponse;

pub async fn register_user(
    new_user: Json<NewUser>,
    data: web::Data<utils::Data>,
) -> HttpAsyncResponse {
    web::block(move || actions::users::create_user(&new_user, &data))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn authenticate_user(
    user: Json<UserLogin>,
    data: web::Data<utils::Data>,
) -> HttpAsyncResponse {
    let result = web::block(move || actions::users::login_user(&user, &data)).await?;

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
