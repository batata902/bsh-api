use crate::{models::models::*, };
use axum::{Json, extract::{self, State, Path, Extension}, http::StatusCode};
use sqlx::SqlitePool;

pub async fn get_user(State(db): State<SqlitePool>, Path(id): Path<i64>, Extension(user): Extension<AuthUser>) -> Result<Json<PublicUser>, (StatusCode, Json<ResponseStatus>)> {
    if id != user.user_id {
        Err((StatusCode::UNAUTHORIZED, Json(ResponseStatus { status: "unauthorized".to_string() })))
    } else {
        let user = sqlx::query_as::<_, PublicUser>("SELECT id, nickname, nickcolor, username FROM users WHERE id=?").bind(&id).fetch_one(&db).await;

        match user {
            Ok(u) => Ok(Json(u)),
            Err(_) => Err((StatusCode::NOT_FOUND, Json(ResponseStatus { status: "not found".to_string() })))
        }
    }
}

pub async fn update_user(Extension(user): Extension<AuthUser>, State(db): State<SqlitePool>, extract::Json(data): extract::Json<UpdateUser>)-> Result<Json<ResponseStatus>, (StatusCode, Json<ResponseStatus>)> {
    match sqlx::query("UPDATE users SET nickname=? WHERE id=?").bind(&data.nickname).bind(&user.user_id).execute(&db).await {
        Ok(_) => Ok(Json(ResponseStatus {status: "ok".to_string()})),
        Err(_) => Err((StatusCode::NOT_FOUND, Json(ResponseStatus { status: "not found".to_string() })))
    }
}

pub async fn delete_user(State(db): State<SqlitePool>, Path(id): Path<u32>) -> Result<Json<ResponseStatus>, (StatusCode, Json<ResponseStatus>)> {
    let consulta = sqlx::query("DELETE FROM users WHERE id=?").bind(id).execute(&db).await;
    match consulta {
        Ok(_) => Ok(Json(ResponseStatus {status: "ok".to_string()})),
        Err(_) => Err((StatusCode::NOT_FOUND, Json(ResponseStatus {status: "not found".to_string()})))
    }
}

pub async fn list_users(State(db): State<SqlitePool>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users").fetch_all(&db).await;

    match users {
        Ok(array) => Ok(Json(array)),
        Err(_) => Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_bolsonaro(State(db): State<SqlitePool>) -> Result<Json<PublicUser>, (StatusCode, Json<ResponseStatus>)> {
    let bolsonaro = sqlx::query_as::<_, PublicUser>("SELECT id, nickname, nickcolor, username FROM users WHERE username='bolsonaro'").fetch_one(&db).await;

    match bolsonaro {
        Ok(bol) => Ok(Json(bol)),
        Err(_) => Err((StatusCode::NOT_FOUND, Json(ResponseStatus { status: "not found".to_string() })))
    }
}