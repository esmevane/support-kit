pub mod assets;
pub mod errors;
pub mod service;
pub mod settings;
pub mod storage;
pub mod telemetry;
pub mod tui;

pub mod contexts {
    pub struct ServiceContext {
        pub operation: crate::settings::ServiceOperation,
        pub settings: crate::settings::Settings,
    }
}

pub type Result<T> = std::result::Result<T, errors::Error>;
