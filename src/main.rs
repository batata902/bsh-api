use axum::{Router, middleware};
use axum::routing::{get, post, delete};
use sqlx::SqlitePool;
use dotenvy::dotenv;
use std::env;
use crate::routes::{middleware as midd, private_routes::*, public_routes::*};
use clap::Parser;

mod routes;
mod utils;
mod models;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value="0.0.0.0")]
    ip: String,
    #[arg(short, long, default_value="9090")]
    port: String
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
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
    .route("/refresh", post(refresh))
    .merge(protected)
    .with_state(db);

    let addr_infos = format!("{}:{}", args.ip, args.port);
    let listener = tokio::net::TcpListener::bind(&addr_infos).await.unwrap();

    println!("Rodando em {} na porta {}", args.ip, args.port);
    println!("http://127.0.0.1:{}", args.port);

    axum::serve(listener, app).await.unwrap();
}
