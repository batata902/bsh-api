use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthUser {
    pub user_id: i64
}

#[derive(Deserialize, FromRow)]
pub struct UserRefresh {
    pub refresh_token: String
}

#[derive(Serialize)]
pub struct ResponseStatus {
    pub status: String
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct LoginUser {
    pub id: i64,
    pub username: String,
    pub password: String
}

#[derive(Deserialize, FromRow, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 10))]
    pub username: String,
    #[validate(length(min = 5, max = 100))]
    pub password: String
}

#[derive(Serialize, FromRow)]
pub struct SetToken {
    pub auth_token: String
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UpdateUser {
    pub id: i64,
    pub nickname: String
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct PublicUser {
    pub id: i64,
    pub nickname: String,
    pub nickcolor: String,
    pub username: String
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 10))]
    pub nickname: String,
    #[validate(length(min = 3, max = 10))]
    pub username: String,
    #[validate(length(min = 5, max = 100))]
    pub password: String
}

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub nickcolor: String,
    pub nickname: String,
    pub username: String,
    pub password: String
}