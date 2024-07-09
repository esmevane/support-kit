pub mod assets;
pub mod errors;
pub mod service;
pub mod settings;
pub mod storage;
pub mod telemetry;
pub mod tui;

pub type Result<T> = std::result::Result<T, errors::Error>;
