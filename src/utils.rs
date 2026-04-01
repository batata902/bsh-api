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

fn now() -> usize {
    SystemTime::now() // Pega horario atual
    .duration_since(UNIX_EPOCH) // Desde o UNIX_EPOCH
    .unwrap() 
    .as_secs() as usize // Pega em segundos e faz Cast para usize
}

pub fn gen_refresh_token(user_id: i64, secret: String) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        is_admin: false,
        exp: (60 * 60 * 60 * 24 * 14)
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

pub async fn store_refresh(db: SqlitePool, refresh_token: &str) {
    let _ = sqlx::query("INSERT INTO users (refresh) VALUES (?)").bind(refresh_token).execute(&db).await;
}

// pub async fn get_password_by_id(db: &SqlitePool, id: i64) -> String {
//     let argon_password = sqlx::query_as::<_, UserPassword>("SELECT password FROM users WHERE id=?;").bind(id).fetch_one(db).await;

//     match argon_password {
//         Ok(password) => password.password,
//         Err(_) => "".to_string()
//     }
// }