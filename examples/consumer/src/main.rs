use support_kit::reexports::{clap::Parser, owo_colors::OwoColorize, tracing};

fn main() -> support_kit::Result<()> {
    let args = support_kit::Args::parse();
    let controller = support_kit::SupportControl::load_configuration(&args)?.init();

    println!("{:#?}", args.bright_green());
    println!("{:#?}", controller.bright_blue());

    tracing::debug!(config = ?controller.config, "loaded configuration");

    match &args.command {
        Some(_) => controller.execute(args)?,
        None => {}
    }

    Ok(())
}
