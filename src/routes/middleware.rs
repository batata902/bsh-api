use axum::{http::{Request, StatusCode, header}, middleware::Next, response::Response};
use crate::{models::models::{AuthUser}, utils::*};
use std::env;

pub async fn auth(mut req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
    let has_token = req.headers().get(header::AUTHORIZATION);
    
    println!("{}: {}", req.method(), req.uri());

    if let Some(token) = has_token {
        let header_str = token.to_str().unwrap_or("");

        if header_str.starts_with("Bearer ") {
            let token = header_str.trim_start_matches("Bearer ");

            match check_jwt(token, env::var("JWT_SECRET").unwrap()) {
                Ok(claims) => {
                    let id = claims.sub.parse::<i64>().map_err(|_| StatusCode::UNAUTHORIZED)?;

                    req.extensions_mut().insert(AuthUser { user_id: id });

                    Ok(next.run(req).await)
                },
                Err(_) => Err(StatusCode::UNAUTHORIZED)
            }
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }

}
