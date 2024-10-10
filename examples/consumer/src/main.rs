use support_kit::reexports::{clap::Parser, owo_colors::OwoColorize};

fn main() -> support_kit::Result<()> {
    let args = support_kit::Args::parse();
    let control = support_kit::SupportControl::load_configuration(&args)?.init();

    println!("{:#?}", args.bright_green());
    println!("{:#?}", control.bright_blue());

    Ok(())
}
