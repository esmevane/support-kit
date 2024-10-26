use clap::Parser;
use owo_colors::OwoColorize;

#[tokio::main]
async fn main() -> support_kit::Result<()> {
    let args = support_kit::Args::parse();
    let controller = support_kit::SupportControl::load_configuration(&args)?.init();

    println!("{:#?}", args.bright_green());
    println!("{:#?}", controller.bright_blue());

    tracing::debug!(config = ?controller.config, "loaded configuration");

    match &args.command {
        Some(_) => controller.execute(args).await?,
        None => {}
    }

    Ok(())
}
