use crate::database::db::DbPool;
use crate::database::models::user::NewDbUser;
use crate::database::models::user::{NewUser, User, UserLogin};
use crate::database::schema::users::dsl as user_dsl;
use crate::utils;
use actix_web::http;
use diesel::prelude::*;

pub fn check_if_exists(nu: &NewUser, pool: &DbPool) -> Result<bool, (http::StatusCode, String)> {
    let conn = pool.get().map_err(|_| {
        (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "errors.database_connection".to_string(),
        )
    })?;

    let exists = user_dsl::users
        .filter(
            user_dsl::email
                .ilike(nu.email.clone())
                .or(user_dsl::username.ilike(nu.username.clone())),
        )
        .first::<User>(&conn)
        .optional()
        .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if exists.is_some() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn create_user(
    nu: &NewUser,
    data: &utils::Data,
    pool: &DbPool,
) -> Result<(), (http::StatusCode, String)> {
    let conn = pool.get().map_err(|_| {
        (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "errors.database_connection".to_string(),
        )
    })?;

    let hash_pass = data
        .hash
        .hash_password(nu.password.as_str())
        .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let db_user = NewDbUser {
        username: nu.username.clone(),
        email: nu.email.clone().to_lowercase(),
        password: hash_pass,
    };

    diesel::insert_into(user_dsl::users)
        .values(db_user)
        .execute(&conn)
        .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(())
}

pub fn login_user(
    ul: &UserLogin,
    data: &utils::Data,
    pool: &DbPool,
) -> Result<User, (http::StatusCode, String)> {
    let conn = pool.get().map_err(|_| {
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

pub fn get_user(user_id: i32, pool: &DbPool) -> Result<User, (http::StatusCode, String)> {
    let conn = pool.get().map_err(|_| {
        (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "errors.database_connection".to_string(),
        )
    })?;

    let exists = user_dsl::users
        .filter(user_dsl::id.eq(user_id))
        .first::<User>(&conn)
        .optional()
        .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match exists {
        Some(u) => Ok(u),
        None => Err((
            http::StatusCode::UNAUTHORIZED,
            "errors.auth.not_found".to_string(),
        )),
    }
}
