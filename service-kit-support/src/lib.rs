pub mod assets;
pub mod errors;
pub mod storage;

pub type Result<T> = std::result::Result<T, errors::Error>;
