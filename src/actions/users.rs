use crate::database::schema::users::dsl::{self as user_dsl};
use crate::database::{
    database::DbError,
    models::user::{NewUser, User, UserLogin},
};
use crate::utils;
use actix_web::http;
use diesel::prelude::*;
use reqwest::StatusCode;

pub fn create_user(nu: &NewUser, data: &utils::Data) -> Result<(), DbError> {
    let conn = data.pool.get()?;

    let exists = user_dsl::users
        .filter(
            user_dsl::email
                .eq(nu.email.clone())
                .or(user_dsl::username.eq(nu.username.clone())),
        )
        .first::<User>(&conn)
        .optional()?;

    if exists.is_some() {
        return Err(Box::new(diesel::NotFound));
    }

    diesel::insert_into(user_dsl::users)
        .values(nu)
        .execute(&conn)?;

    Ok(())
}

pub fn login_user(ul: &UserLogin, data: &utils::Data) -> Result<User, (http::StatusCode, String)> {
    let conn = data.pool.get().map_err(|_| {
        (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "errors.database_connection".to_string(),
        )
    })?;

    let exists = user_dsl::users
        .filter(user_dsl::email.eq(ul.email.clone()))
        .first::<User>(&conn)
        .optional()
        .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = match exists {
        Some(u) => u,
        None => {
            return Err((
                http::StatusCode::UNAUTHORIZED,
                "errors.auth.not_found".to_string(),
            ))
        }
    };

    match data.hash.verify_password(&ul.password, &user.password) {
        Ok(true) => Ok(user),
        _ => Err((
            http::StatusCode::UNAUTHORIZED,
            "errors.auth.invalid_credentials".to_string(),
        )),
    }
}
