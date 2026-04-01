use crate::models::models::*;
use crate::utils::*;
use std::env;
use axum::{Json, extract::{self, State}, http::StatusCode};
use sqlx::SqlitePool;
use validator::Validate;

pub async fn index() -> String {
    String::from("A api está no ar.")
}

pub async fn create_user(State(db): State<SqlitePool>, extract::Json(data): extract::Json<CreateUser>) -> Result<Json<ResponseStatus>, StatusCode> {
    if let Err(_) = data.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

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
    if let Err(_) = data.validate() {
        return Err((StatusCode::BAD_REQUEST, Json(ResponseStatus { status: "bad request".to_string() })));
    }

    let logged_in = sqlx::query_as::<_, LoginUser>("SELECT id, username, password FROM users WHERE username=?;").bind(&data.username).fetch_one(&db).await;

    match logged_in {
        Ok(user_data) => {
            let check = verify_password(&data.password, &user_data.password);

            println!("{}", check);

            if check {
                let refresh_token = gen_refresh_token(user_data.id, env::var("JWT_SECRET").unwrap());
                store_refresh(db, refresh_token.as_str()).await;
                Ok(Json(SetToken {auth_token: gen_jwt(user_data.id, env::var("JWT_SECRET").unwrap())}))
            } else {
                Err((StatusCode::UNAUTHORIZED, Json(ResponseStatus { status: "unauthorized".to_string()})))
            }
        },
        Err(e) => {
            println!("{}", e);
            Err((StatusCode::UNAUTHORIZED, Json(ResponseStatus {status: "unauthorized".to_string()})))
        }
    }
}