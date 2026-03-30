use jsonwebtoken::{EncodingKey, encode, Header, DecodingKey, decode, Validation, errors::Error};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String, // Vamos guardar o id do usuario aqui
    pub is_admin: bool,
    exp: usize
}

fn now() -> usize {
    SystemTime::now() // Pega horario atual
    .duration_since(UNIX_EPOCH) // Desde o UNIX_EPOCH
    .unwrap() 
    .as_secs() as usize // Pega em segundos e faz Cast para usize
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