use crate::{models::models::*, };
use axum::{Json, extract::{self, Extension, Path, State}, http::StatusCode};
use sqlx::SqlitePool;
use crate::utils::is_admin;

pub async fn get_user(State(db): State<SqlitePool>, Path(id): Path<i64>, Extension(user): Extension<AuthUser>) -> Result<Json<PublicUser>, (StatusCode, Json<ResponseStatus>)> {
    if id != user.user_id && !is_admin(&db, user.user_id).await {
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

pub async fn delete_user(State(db): State<SqlitePool>, Path(id): Path<i64>, Extension(user): Extension<AuthUser>) -> Result<Json<ResponseStatus>, (StatusCode, Json<ResponseStatus>)> {
    if user.user_id == id || is_admin(&db, user.user_id).await {
        let consulta = sqlx::query("DELETE FROM users WHERE id=?").bind(id).execute(&db).await;
        
        match consulta {
            Ok(_) => Ok(Json(ResponseStatus {status: "ok".to_string()})),
            Err(e) => {
                println!("Error: {}", e);
                Err((StatusCode::NOT_FOUND, Json(ResponseStatus {status: "not found".to_string()})))
            }
        }
    } else {
        Err((StatusCode::UNAUTHORIZED, Json(ResponseStatus { status: "unauthorized".to_string() })))
    }
}

pub async fn list_users(State(db): State<SqlitePool>, Extension(user): Extension<AuthUser>) -> Result<Json<Vec<User>>, StatusCode> {
    if is_admin(&db, user.user_id).await {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users").fetch_all(&db).await;

        match users {
            Ok(array) => Ok(Json(array)),
            Err(_) => Err(StatusCode::NOT_FOUND)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn send_post(State(db): State<SqlitePool>, Extension(user): Extension<AuthUser>,extract::Json(data): extract::Json<SendPost>) -> Result<Json<ResponseStatus>, StatusCode> {
    let query = sqlx::query("INSERT INTO posts (author, content) VALUES (?, ?);").bind(&user.user_id).bind(&data.content).execute(&db).await;

    match query {
        Ok(_) => Ok(Json( ResponseStatus { status: "ok".to_string() } )),
        Err(_) => Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn all_posts(State(db): State<SqlitePool>) -> Result<Json<Vec<Posts>>, StatusCode> {
    let posts = sqlx::query_as::<_, Posts>("SELECT p.id, u.nickname, p.content, u.nickcolor, p.data_post FROM posts p JOIN users u ON u.id = p.author;").fetch_all(&db).await;

    match posts {
        Ok(p) => Ok(Json(p)),
        Err(e) => {
            println!("Erro: {}", e);
            Err(StatusCode::NO_CONTENT)
        }
    }
}

pub async fn get_bolsonaro(State(db): State<SqlitePool>) -> Result<Json<PublicUser>, (StatusCode, Json<ResponseStatus>)> {
    let bolsonaro = sqlx::query_as::<_, PublicUser>("SELECT id, nickname, nickcolor, username FROM users WHERE username='bolsonaro'").fetch_one(&db).await;

    match bolsonaro {
        Ok(bol) => Ok(Json(bol)),
        Err(_) => Err((StatusCode::NOT_FOUND, Json(ResponseStatus { status: "not found".to_string() })))
    }
}