use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Vamos guardar o id do usuario aqui
    pub is_admin: bool,
    pub exp: usize
}

#[derive(Serialize, Deserialize)]
pub struct RefreshClaim {
    pub sub: String,
    pub exp: usize
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthUser {
    pub user_id: i64
}

#[derive(Deserialize, FromRow)]
pub struct UserRefresh {
    pub refresh: String
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
    pub auth_token: String,
    pub refresh_token: String
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UpdateUser {
    pub id: i64,
    pub nickname: String,
    pub nickcolor: String
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
    pub role: String,
    #[validate(length(min = 3, max = 10))]
    pub nickname: String,
    #[validate(length(min = 3, max = 10))]
    pub username: String,
    #[validate(length(min = 5, max = 100))]
    pub password: String
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Posts {
    pub id: i64,
    pub data_post: String,
    pub nickname: String,
    pub content: String,
    pub nickcolor: String
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct SendPost {
    pub content: String
}

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub role: String,
    pub refresh: String,
    pub nickcolor: String,
    pub nickname: String,
    pub username: String
}