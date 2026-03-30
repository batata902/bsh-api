use axum::{Json, Router, extract::{self, Path, State}, http::StatusCode};
use axum::routing::{get, post, delete};

use serde::{Serialize, Deserialize};
use sqlx::{SqlitePool, prelude::FromRow};

#[derive(Serialize)]
struct ResponseStatus {
    status: String
}

#[derive(Serialize, Deserialize, FromRow)]
struct LoginUser {
    username: String,
    password: String
}

#[derive(Serialize, FromRow)]
struct SetCookie {
    auth_token: String
}

#[derive(Serialize, Deserialize, FromRow)]
struct UpdateUser {
    id: i64,
    nickname: String
}

#[derive(Serialize, Deserialize, FromRow)]
struct PublicUser {
    id: i64,
    nickname: String,
    nickcolor: String,
    username: String
}

#[derive(Serialize, Deserialize)]
struct CreateUser {
    nickname: String,
    username: String,
    password: String
}

#[tokio::main]
async fn main() {
    let db = SqlitePool::connect("sqlite://database.db").await.unwrap();

    let app = Router::new()
    .route("/", get(index))
    .route("/api/create", post(create_user))
    .route("/user/:id", get(get_user))
    .route("/user/:id", delete(delete_user))
    .route("/update", post(update_user))
    .route("/bolsonaro", get(get_bolsonaro))
    .route("/login", post(login))
    .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9090").await.unwrap();

    println!("Rodando em 127.0.0.1 na porta 9090");
    println!("http://127.0.0.1:9090");

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> String {
    format!("A api está no ar.")
}

async fn create_user(State(db): State<SqlitePool>, extract::Json(data): extract::Json<CreateUser>) -> Json<ResponseStatus> {
    sqlx::query("INSERT INTO users (nickname, username, password) VALUES (?, ?, ?);")
    .bind(&data.nickname)
    .bind(&data.username)
    .bind(&data.password)
    .execute(&db)
    .await
    .unwrap();

    Json(ResponseStatus { status: "ok".to_string() })
}


async fn get_user(State(db): State<SqlitePool>, Path(id): Path<u32>) -> Result<Json<PublicUser>, (StatusCode, Json<ResponseStatus>)> {
    let user = sqlx::query_as::<_, PublicUser>("SELECT id, nickname, nickcolor, username, auth_token FROM users WHERE id=?").bind(&id).fetch_one(&db).await;

    match user {
        Ok(u) => Ok(Json(u)),
        Err(_) => Err((StatusCode::NOT_FOUND, Json(ResponseStatus { status: "not found".to_string() })))
    }
}

async fn update_user(State(db): State<SqlitePool>, extract::Json(data): extract::Json<UpdateUser>)-> Result<Json<ResponseStatus>, (StatusCode, Json<ResponseStatus>)> {
    match sqlx::query("UPDATE users SET nickname=? WHERE id=?").bind(&data.nickname).bind(&data.id).execute(&db).await {
        Ok(_) => Ok(Json(ResponseStatus {status: "ok".to_string()})),
        Err(_) => Err((StatusCode::NOT_FOUND, Json(ResponseStatus { status: "not found".to_string() })))
    }
}

async fn delete_user(State(db): State<SqlitePool>, Path(id): Path<u32>) -> Result<Json<ResponseStatus>, (StatusCode, Json<ResponseStatus>)> {
    let consulta = sqlx::query("DELETE FROM users WHERE id=?").bind(id).execute(&db).await;
    match consulta {
        Ok(_) => Ok(Json(ResponseStatus {status: "ok".to_string()})),
        Err(_) => Err((StatusCode::NOT_FOUND, Json(ResponseStatus {status: "not found".to_string()})))
    }
}

async fn login(State(db): State<SqlitePool>, extract::Json(data): extract::Json<LoginUser>) -> Result<Json<SetCookie>, (StatusCode, Json<ResponseStatus>)> {
    let logged_in = sqlx::query_as::<_, LoginUser>("SELECT * FROM users WHERE username=? AND password=?;").bind(&data.username).bind(&data.password).fetch_one(&db).await;

    match logged_in {
        Ok(_) => Ok(Json(SetCookie { auth_token: "cookiefodao".to_string() })),
        Err(_) => Err((StatusCode::UNAUTHORIZED, Json(ResponseStatus {status: "forbbiden".to_string()})))
    }
}

async fn get_bolsonaro(State(db): State<SqlitePool>) -> Result<Json<PublicUser>, (StatusCode, Json<ResponseStatus>)> {
    let bolsonaro = sqlx::query_as::<_, PublicUser>("SELECT id, nickname, nickcolor, username FROM users WHERE username='Bolsonaro'").fetch_one(&db).await;

    match bolsonaro {
        Ok(bol) => Ok(Json(bol)),
        Err(_) => Err((StatusCode::NOT_FOUND, Json(ResponseStatus { status: "not found".to_string() })))
    }
}
