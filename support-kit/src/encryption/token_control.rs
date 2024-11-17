use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use secrecy::{ExposeSecret, SecretString};

use crate::{AuthTokenGenerationFailure, AuthTokenVerificationFailure, Configuration, TokenError};

/// The claims that our json web token can make.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct TokenContents {
    /// The expected subject of the token. This is the user's ID.
    pub sub: String,
    /// The expected expiration of the token.
    pub exp: usize,
}

pub struct TokenControl(pub Configuration);

impl TokenControl {
    pub fn from_config(config: Configuration) -> Self {
        Self(config)
    }

    pub fn auth_token(&self) -> crate::Result<String> {
        let id = uuid::Uuid::new_v4();
        let secret = self.0.secret.clone();

        Ok(generate_auth_token(&id, &secret)?)
    }

    pub fn validate_auth_token(&self, token: String) -> crate::Result<uuid::Uuid> {
        let secret = self.0.secret.clone();

        Ok(validate_auth_token(token, &secret)?)
    }

    pub fn random(&self) -> String {
        generate_randomized_token()
    }
}

/// Generate a JSON web token for auth.
#[tracing::instrument(name = "Generating session auth token", skip(id, secret))]
pub fn generate_auth_token(id: &uuid::Uuid, secret: &SecretString) -> Result<String, TokenError> {
    let header = Header {
        kid: Some(secret.expose_secret().to_owned()),
        alg: Algorithm::HS512,
        ..Default::default()
    };

    let claim = TokenContents {
        sub: id.to_string(),
        exp: 10000000000,
    };

    Ok(encode(
        &header,
        &claim,
        &EncodingKey::from_secret(secret.expose_secret().as_bytes()),
    )
    .map_err(AuthTokenGenerationFailure::from)?)
}

/// Validate a JSON web token.
#[tracing::instrument(
    level = "debug",
    name = "Validate session auth token",
    skip(token, secret)
)]
pub fn validate_auth_token(token: String, secret: &SecretString) -> Result<uuid::Uuid, TokenError> {
    let raw_token = decode::<TokenContents>(
        &token,
        &DecodingKey::from_secret(secret.expose_secret().as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS512),
    )
    .map_err(AuthTokenVerificationFailure::from)?
    .claims
    .sub;

    Ok(uuid::Uuid::parse_str(&raw_token)?)
}

/// Generates a random token for use in session tokens.
pub fn generate_randomized_token() -> String {
    let mut rng = rand::thread_rng();

    std::iter::repeat_with(|| rng.sample(rand::distributions::Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}
