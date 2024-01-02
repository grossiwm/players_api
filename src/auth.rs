use std::{time::{SystemTime, UNIX_EPOCH}, env};

use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

pub trait Authenticable {
    fn get_username(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}


pub fn authorize_token(token: &str) -> Result<Claims, &'static str> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret_key().as_ref()),
        &Validation::new(Algorithm::HS256)
    )
    .map(|data| data.claims)
    .map_err(|_| "Invalid or expired token")
}

fn get_secret_key() -> String {
    env::var("JWT_SECRET_KEY").unwrap_or_else(|_| String::from("default_secret"))
}

pub fn generate_jwt(authenticable: impl Authenticable) -> Result<String, &'static str> {
    let secret_key = get_secret_key();

    let duration = 60 * 60;
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize + duration;

    let claims = Claims {
        sub: authenticable.get_username(), 
        exp: expiration,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
        .map_err(|_| "Falha ao gerar o token")?;

    Ok(token)
}
