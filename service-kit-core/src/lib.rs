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

pub type Result<T> = color_eyre::eyre::Result<T, Error>;
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

shadow!(build);

pub async fn run() -> Result<()> {
    settings::Settings::parse()?.exec().await?;

    Ok(())
}
