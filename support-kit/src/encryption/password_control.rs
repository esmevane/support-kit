use crate::{Configuration, Environment, PasswordError};
use argon2::{password_hash::SaltString, Argon2, Params, PasswordHasher, PasswordVerifier};

pub struct PasswordControl {
    params: Params,
}

impl PasswordControl {
    pub fn test() -> Self {
        let params = Params::new(8, 1, 1, Some(32)).unwrap();
        Self { params }
    }

    pub fn from_config(config: Configuration) -> Self {
        if let Some(Environment::Test) = config.environment {
            Self::test()
        } else {
            Self::default()
        }
    }

    pub fn hasher(&self) -> Argon2<'static> {
        Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            self.params.clone(),
        )
    }

    /// Password generation creates an argon2 password hash from a given password.
    #[tracing::instrument(level = "debug", name = "Generate password", skip(self, password))]
    pub fn generate_password_hash(&self, password: &str) -> crate::Result<String> {
        let salt = SaltString::generate(&mut rand::thread_rng());

        Ok(self
            .hasher()
            .hash_password(password.as_bytes(), &salt)
            .map_err(PasswordError::from)?
            .to_string())
    }

    /// Validate a password against a password hash.
    #[tracing::instrument(
        level = "debug",
        name = "Verifying password",
        skip(self, password, password_hash)
    )]
    pub fn validate_password_hash(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<(), argon2::password_hash::Error> {
        let password = String::from(password);
        let password_hash = String::from(password_hash);

        let hash = argon2::PasswordHash::new(&password_hash)?;

        self.hasher().verify_password(password.as_bytes(), &hash)
    }
}

impl Default for PasswordControl {
    fn default() -> Self {
        let params = Params::default();
        Self { params }
    }
}
