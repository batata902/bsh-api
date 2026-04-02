use jsonwebtoken::{EncodingKey, encode, Header, DecodingKey, decode, Validation, errors::Error};
use serde::{Serialize, Deserialize};
use std::{time::{SystemTime, UNIX_EPOCH}};
use argon2::{Argon2, password_hash::{SaltString, PasswordHasher, PasswordHash, PasswordVerifier}};
use rand_core::OsRng;
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Vamos guardar o id do usuario aqui
    pub is_admin: bool,
    pub exp: usize
}

#[derive(Serialize, Deserialize)]
pub struct RefreshClaim {
    pub sub: String,
    pub exp: usize
}

fn now() -> usize {
    SystemTime::now() // Pega horario atual
    .duration_since(UNIX_EPOCH) // Desde o UNIX_EPOCH
    .unwrap() 
    .as_secs() as usize // Pega em segundos e faz Cast para usize
}

pub fn gen_refresh_token(user_id: i64, secret: String) -> String {
    let claims = RefreshClaim {
        sub: user_id.to_string(),
        exp: now() + (60 * 60 * 24 * 14)
    };

    encode(&Header::default(), 
    &claims, 
    &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn gen_jwt(user_id: i64, secret: String) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        is_admin: false,
        exp: now() + 1800 // 30 minutos
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn check_refresh(token: &str, secret: String) -> Result<RefreshClaim, Error> {
    let valido = decode(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())?;

    Ok(valido.claims)
}

pub fn check_jwt(token: &str, secret: String) -> Result<Claims, Error> {
    let valido = decode(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())?;

    Ok(valido.claims)
}

pub fn get_hash(data: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
    .hash_password(data.as_bytes(), &salt)
    .unwrap()
    .to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => {
            return false;
        }
    };

    match Argon2::default()
    .verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => {
            true
        },
        Err(_) => {
            false
        }
    }
}

pub async fn store_refresh(db: &SqlitePool, refresh_token: &str, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET refresh=? WHERE id=?")
    .bind(refresh_token)
    .bind(id)
    .execute(db)
    .await?;

    Ok(())
}
