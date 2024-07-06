use shadow_rs::shadow;

mod cli;
mod client;
mod context;
mod errors;
mod server;
mod service;
mod settings;
mod telemetry;
mod tui;

pub use client::WebClient;
pub use context::WebContext;
pub use errors::Error;
use tracing_appender::non_blocking::WorkerGuard;

pub type Result<T> = color_eyre::eyre::Result<T, Error>;
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

shadow!(build);

pub async fn run() -> Result<WorkerGuard> {
    let (settings, guard) = settings::Settings::parse()?;

    settings.exec().await?;

    Ok(guard)
}
