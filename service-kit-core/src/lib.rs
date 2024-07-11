use clap::Parser;
use shadow_rs::shadow;

mod cli;
mod client;
mod context;
mod errors;
mod server;
mod service;
mod tui;

pub use client::WebClient;
pub use context::WebContext;
pub use errors::Error;
use tracing_appender::non_blocking::WorkerGuard;

pub type Result<T> = color_eyre::eyre::Result<T, Error>;
pub const APP_NAME: &str = "service-kit";

shadow!(build);

pub async fn run() -> Result<Vec<WorkerGuard>> {
    let (settings, guards) = service_kit_support::settings::Settings::parse(cli::Cli::parse())?;

    // settings.exec().await?;

    Ok(guards)
}
