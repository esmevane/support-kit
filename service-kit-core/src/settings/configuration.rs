mod logging;

use serde::Deserialize;
use std::path::PathBuf;

use crate::APP_NAME;

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(default)]
pub struct Configuration {
    pub db: Database,
    pub storage: Storage,
    pub logging: logging::Logging,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Storage {
    pub path: PathBuf,
}

impl Default for Storage {
    fn default() -> Self {
        // use directories to get a default data directory in user's config path
        let path = match dirs::config_local_dir() {
            Some(mut path) => {
                path.push(APP_NAME.to_lowercase());
                path.push("storage.db");
                path
            }
            None => {
                // otherwise we start in a temp directory
                std::env::temp_dir()
                    .join(APP_NAME.to_lowercase())
                    .join("storage.db")
            }
        };

        tracing::debug!("Using storage path: {}", path.display());

        // ensure the file and path exist
        if let Some(parent) = path.parent() {
            tracing::debug!("Ensuring storage path exists: {:?}", parent);
            std::fs::create_dir_all(parent).expect("Unable to create storage directory");
        }

        if std::fs::metadata(&path).is_err() {
            tracing::debug!("Ensuring storage file exists: {:?}", path);
            std::fs::File::create(&path).expect("Unable to create storage file");
        }

        Self { path }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: Option<String>,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 3306,
            user: "root".to_string(),
            password: "password".to_string(),
            database: None,
        }
    }
}

impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mysql://{}:{}@{}:{}/{}",
            self.user,
            self.password,
            self.host,
            self.port,
            self.database.as_deref().unwrap_or(&APP_NAME),
        )
    }
}
