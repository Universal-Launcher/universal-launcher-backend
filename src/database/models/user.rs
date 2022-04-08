use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::database::schema::users;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, Identifiable)]
#[primary_key(id)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub email: String,
    pub email_verified: bool,
}

#[derive(Debug, Clone, Insertable, Validate, Deserialize)]
#[table_name = "users"]
pub struct NewDbUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 1, message = "errors.validation.username.required"))]
    pub username: String,
    #[validate(length(min = 1, message = "errors.validation.password.required"))]
    #[validate(email(message = "errors.validation.email.invalid"))]
    pub email: String,
    #[validate(length(min = 1, message = "errors.validation.password.required"))]
    #[validate(length(min = 8, message = "errors.validation.password.length"))]
    pub password: String,
    #[validate(length(min = 1, message = "errors.validation.confirmation.required"))]
    #[validate(must_match(
        other = "password",
        message = "errors.validation.confirmation.must_match"
    ))]
    pub confirmation: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct UserLogin {
    #[validate(email(message = "errors.validation.email.invalid"))]
    pub email: String,
    // Since `required` doesn't accept messages, we've to bypass by asking
    // a message having at least one character.
    #[validate(length(min = 1, message = "errors.validation.password.required"))]
    pub password: String,
}
