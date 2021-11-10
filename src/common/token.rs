use serde::{Serialize, Deserialize,};
use crate::{ GLOBAL_CONF};
use jsonwebtoken::{decode, TokenData, Algorithm, DecodingKey, Validation, encode, EncodingKey, Header};
use anyhow::Result;


pub const  HTTP_TOKEN_KEY: &str = "AUTH_TOKEN";


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub phone: i64,
    pub username: String,
    pub exp: usize,
}

pub fn token_data(token: &str) -> Result<TokenData<Claims>> {
    let site_key = GLOBAL_CONF.jwt.site_key.as_bytes();
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(site_key),
        &Validation::new(Algorithm::HS512),
    )?;
    Ok(data)
}
pub fn generate_token(data: &Claims) -> Result<String> {
    let mut header = Header::default();
    header.alg = Algorithm::HS512;

    let site_key = GLOBAL_CONF.jwt.site_key.as_bytes();
    let claims = data;
    let token = encode(&header, &claims, &EncodingKey::from_secret(site_key),)?;
    Ok(token)
}
