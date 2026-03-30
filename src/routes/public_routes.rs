use crate::models::models::*;
use crate::utils::*;
use std::env;
use axum::{Json, extract::{self, State}, http::StatusCode};
use sqlx::SqlitePool;

pub async fn index() -> String {
    format!("A api está no ar.")
}

pub async fn create_user(State(db): State<SqlitePool>, extract::Json(data): extract::Json<CreateUser>) -> Result<Json<ResponseStatus>, StatusCode> {
    let consulta = sqlx::query("INSERT INTO users (nickname, username, password) VALUES (?, ?, ?);")
    .bind(&data.nickname)
    .bind(&data.username)
    .bind(get_hash(&data.password))
    .execute(&db)
    .await;

    match consulta {
        Ok(_) => Ok(Json(ResponseStatus { status: "ok".to_string() })),
        Err(_) => Err(StatusCode::CONFLICT)
    }
}

pub async fn login(State(db): State<SqlitePool>, extract::Json(data): extract::Json<LoginRequest>) -> Result<Json<SetToken>, (StatusCode, Json<ResponseStatus>)> {
    let logged_in = sqlx::query_as::<_, LoginUser>("SELECT id, username, password FROM users WHERE username=?;").bind(&data.username).fetch_one(&db).await;

    match logged_in {
        Ok(user_data) => {
            if user_data.password == get_hash(&data.password) {
                Ok(Json(SetToken {auth_token: gen_jwt(user_data.id, env::var("JWT_SECRET").unwrap())}))
            } else {
                Err((StatusCode::UNAUTHORIZED, Json(ResponseStatus { status: "unauthorized".to_string() })))
            }
        },
        Err(_) => Err((StatusCode::UNAUTHORIZED, Json(ResponseStatus {status: "unauthorized".to_string()})))
    }
}