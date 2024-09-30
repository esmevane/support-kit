use clap::Parser;

pub fn main() {
    let args = support_kit::Args::parse();

    let _logging = args.as_config().init_logging();

    tracing::trace!("Hello, world!");
    tracing::debug!("Hello, world!");
    tracing::info!("Hello, world!");
    tracing::warn!("Hello, world!");
    tracing::error!("Hello, world!");
}
