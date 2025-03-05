use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use std::env;
use dotenvy::dotenv;
/// Structure representing JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,        
    pub username: String,  
    pub role:String,
    pub exp: usize,        
}

/// Generates a JWT token for a given user
pub fn generate_jwt(user_id: &str, username: &str,role:&str) -> Result<String, Error> {
    dotenv().ok();
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let expiration = Utc::now() + Duration::hours(8); 
    let claims = Claims {
        id: user_id.to_string(),
        username: username.to_string(),
        role:role.to_string(),
        exp: expiration.timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
}

/// Validates a JWT token and extracts the user information
pub fn validate_jwt(token: &str) -> Result<Claims, Error> {
    dotenv().ok();
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}
