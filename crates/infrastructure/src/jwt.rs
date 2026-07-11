use crate::auth::{Claims, User};
use odin_kernel::KernelError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct JwtService {
    secret: Vec<u8>,
    expiration_secs: u64,
}

impl std::fmt::Debug for JwtService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtService")
            .field("expiration_secs", &self.expiration_secs)
            .finish()
    }
}

impl Clone for JwtService {
    fn clone(&self) -> Self {
        Self {
            secret: self.secret.clone(),
            expiration_secs: self.expiration_secs,
        }
    }
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.as_bytes().to_vec(),
            expiration_secs: 86400,
        }
    }

    pub fn with_expiration(mut self, secs: u64) -> Self {
        self.expiration_secs = secs;
        self
    }

    pub fn generate_token(&self, user: &User) -> Result<String, KernelError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| KernelError::Internal(format!("Time error: {}", e)))?
            .as_secs() as usize;

        let claims = Claims {
            sub: user.id.clone(),
            username: user.username.clone(),
            roles: user.roles.iter().map(|r| format!("{:?}", r)).collect(),
            exp: now + self.expiration_secs as usize,
            iat: now,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(&self.secret))
            .map_err(|e| KernelError::Internal(format!("Token generation failed: {}", e)))
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, KernelError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &Validation::default(),
        )
        .map_err(|e| KernelError::Internal(format!("Token validation failed: {}", e)))?;

        Ok(token_data.claims)
    }
}
