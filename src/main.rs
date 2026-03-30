use axum::{Json, 
    Router, 
    extract::{self, Path, State}, 
    http::{StatusCode, Request},
    response::Response,
    middleware::{self, Next}};
use axum::routing::{get, post, delete};
use serde::{Serialize, Deserialize};
use sqlx::{SqlitePool, prelude::FromRow};
use crate::utils::{check_jwt, gen_jwt, get_hash};
use dotenvy::dotenv;
use std::env;

mod utils;

#[derive(Serialize)]
struct ResponseStatus {
    status: String
}

#[derive(Serialize, Deserialize, FromRow)]
struct LoginUser {
    id: i64,
    username: String,
    password: String
}

#[derive(Deserialize, FromRow)]
struct LoginRequest {
    username: String,
    password: String
}

#[derive(Serialize, FromRow)]
struct SetToken {
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

#[derive(Serialize, FromRow)]
struct User {
    id: i64,
    nickcolor: String,
    nickname: String,
    username: String,
    password: String
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();

    let db = SqlitePool::connect(&database_url).await.unwrap();

    let protected = Router::new()
    .route("/user/:id", get(get_user))
    .route("/user/:id", delete(delete_user))
    .route("/update", post(update_user))
    .route("/users", get(list_users))
    .route("/bolsonaro", get(get_bolsonaro))
    .layer(middleware::from_fn(auth));

    let app = Router::new()
    .route("/user", post(create_user))
    .route("/", get(index))
    .route("/login", post(login))
    .merge(protected)
    .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9090").await.unwrap();

    println!("Rodando em 127.0.0.1 na porta 9090");
    println!("http://127.0.0.1:9090");

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> String {
    format!("A api está no ar.")
}

async fn auth(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
    let has_token = req.headers().get("authorization");

    println!("{}: {}", req.method(), req.uri());

    // Padrão nos headers -> Authorization: Bearer TOKEN
    if let Some(token) = has_token {
        let header_str = token.to_str().unwrap_or("");

        if header_str.starts_with("Bearer ") {
            // Remove o "Bearer " da string
            let token = header_str.trim_start_matches("Bearer "); 
            match check_jwt(token, env::var("JWT_SECRET").unwrap()) {
                Ok(claims) => {
                    if claims.is_admin == true {
                        Ok(next.run(req).await)
                    } else {
                        Err(StatusCode::UNAUTHORIZED)
                    }
                },
                Err(_) => Err(StatusCode::UNAUTHORIZED)
            }
        }
        else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }

}

async fn create_user(State(db): State<SqlitePool>, extract::Json(data): extract::Json<CreateUser>) -> Result<Json<ResponseStatus>, StatusCode> {
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


async fn get_user(State(db): State<SqlitePool>, Path(id): Path<u32>) -> Result<Json<PublicUser>, (StatusCode, Json<ResponseStatus>)> {
    let user = sqlx::query_as::<_, PublicUser>("SELECT id, nickname, nickcolor, username FROM users WHERE id=?").bind(&id).fetch_one(&db).await;

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

async fn login(State(db): State<SqlitePool>, extract::Json(data): extract::Json<LoginRequest>) -> Result<Json<SetToken>, (StatusCode, Json<ResponseStatus>)> {
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

// FOR DEV

async fn list_users(State(db): State<SqlitePool>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users").fetch_all(&db).await;

    match users {
        Ok(array) => Ok(Json(array)),
        Err(_) => Err(StatusCode::NOT_FOUND)
    }
}

async fn get_bolsonaro(State(db): State<SqlitePool>) -> Result<Json<PublicUser>, (StatusCode, Json<ResponseStatus>)> {
    let bolsonaro = sqlx::query_as::<_, PublicUser>("SELECT id, nickname, nickcolor, username FROM users WHERE username='Bolsonaro'").fetch_one(&db).await;

    match bolsonaro {
        Ok(bol) => Ok(Json(bol)),
        Err(_) => Err((StatusCode::NOT_FOUND, Json(ResponseStatus { status: "not found".to_string() })))
    }
}
