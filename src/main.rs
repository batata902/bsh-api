use axum::{Router, middleware};
use axum::routing::{get, post, delete};
use sqlx::SqlitePool;
use dotenvy::dotenv;
use std::env;
use crate::routes::{middleware as midd, private_routes::*, public_routes::*};

mod routes;
mod utils;
mod models;

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
    .layer(middleware::from_fn(midd::auth));

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
