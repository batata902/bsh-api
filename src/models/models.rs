use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;


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

#[derive(Deserialize, FromRow)]
pub struct LoginRequest {
    pub username: String,
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

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub nickname: String,
    pub username: String,
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