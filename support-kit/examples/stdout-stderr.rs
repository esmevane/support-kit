use clap::Parser;

pub fn main() {
    let args = support_kit::Args::parse();

    let _logging = args.as_config().init_logging();

    tracing::trace!("Only shows up if you use --verbose x5 or -vvvvv");
    tracing::debug!("Only shows up if you use --verbose x4 or -vvvv");
    tracing::info!("Only shows up if you use --verbose x3 or -vvv");
    tracing::warn!("Only shows up if you use --verbose x2 or -vv");
    tracing::error!("Only shows up if you use --verbose or -v");
}
