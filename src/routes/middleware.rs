use axum::{http::{Request, StatusCode}, middleware::Next, response::Response};
use crate::utils::*;
use std::env;

pub async fn auth(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
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
