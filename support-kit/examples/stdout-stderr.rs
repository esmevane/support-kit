use clap::Parser;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = support_kit::Args::parse();
    let control = support_kit::SupportControl::load_configuartion(&args)?.init();

    tracing::trace!("Only shows up if you use --verbose x5 or -vvvvv");
    tracing::debug!("Only shows up if you use --verbose x4 or -vvvv");
    tracing::info!("Only shows up if you use --verbose x3 or -vvv");
    tracing::warn!("Only shows up if you use --verbose x2 or -vv");
    tracing::error!("Only shows up if you use --verbose or -v");

    control.execute(args).expect("failed to execute control");

    Ok(())
}
