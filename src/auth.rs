use std::{time::{SystemTime, UNIX_EPOCH}, env};

use crate::player::{Claims, Player};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, Algorithm};

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

pub fn generate_jwt(player: Player) -> Result<String, &'static str> {
    let secret_key = get_secret_key();

    let duration = 60 * 60;
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize + duration;

    let claims = Claims {
        sub: player.username.to_string(), 
        exp: expiration,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
        .map_err(|_| "Falha ao gerar o token")?;

    Ok(token)
}
