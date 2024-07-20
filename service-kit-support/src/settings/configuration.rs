use serde::Deserialize;
use std::path::PathBuf;

use super::{Logging, Network, Service, Settings};

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(default)]
pub struct Configuration {
    // pub db: Database,
    pub storage: Storage,
    pub logging: Logging,
    pub server: Network,
    pub client: Network,
    pub service: Service,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Storage {
    pub path: PathBuf,
    pub name: Option<String>,
}

impl Storage {
    pub fn initialize(&self, settings: &Settings) -> crate::Result<Self> {
        let name = self.name.clone().unwrap_or("storage.db".to_string());

        // use directories to get a default data directory in user's config path
        let path = match dirs::config_local_dir() {
            Some(mut path) => {
                path.push(settings.app_name.to_lowercase());
                path.push(&name);
                path
            }
            None => {
                // otherwise we start in a temp directory
                std::env::temp_dir()
                    .join(settings.app_name.to_lowercase())
                    .join(&name)
            }
        };

        tracing::debug!("Using storage path: {}", path.display());

        // ensure the file and path exist
        if let Some(parent) = path.parent() {
            tracing::debug!("Ensuring storage path exists: {:?}", parent);
            std::fs::create_dir_all(parent)?;
        }

        if std::fs::metadata(&path).is_err() {
            tracing::debug!("Ensuring storage file exists: {:?}", path);
            std::fs::File::create(&path)?;
        }

        Ok(Self {
            path,
            name: Some(name),
        })
    }
}

// #[derive(Clone, Debug, Deserialize)]
// pub struct Database {
//     host: String,
//     port: u16,
//     user: String,
//     password: String,
//     database: Option<String>,
// }

// impl Default for Database {
//     fn default() -> Self {
//         Self {
//             host: "localhost".to_string(),
//             port: 3306,
//             user: "root".to_string(),
//             password: "password".to_string(),
//             database: None,
//         }
//     }
// }

// impl Database {
//     fn connection_string(&self, settings: &Settings) -> String {
//         format!(
//             "mysql://{}:{}@{}:{}/{}",
//             self.user,
//             self.password,
//             self.host,
//             self.port,
//             self.database.as_deref().unwrap_or(&settings.app_name),
//         )
//     }
// }
