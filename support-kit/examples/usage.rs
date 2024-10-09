use clap::{Parser, Subcommand};
use support_kit::SupportControl;

#[derive(Debug, Parser)]
struct YourOwnCli {
    #[clap(subcommand)]
    command: Option<LocalCommand>,

    #[clap(flatten)]
    support: support_kit::Args,
}

#[derive(Clone, Copy, Debug, Subcommand, PartialEq)]
enum LocalCommand {
    Local,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // You can use your own CLI struct here and parse as normal.
    let cli = YourOwnCli::parse();

    // Construct a support controller with the support-kit CLI arguments.
    // "Init" activates any setup, like log tracing, so after this point you can
    // use support kit managed tooling.
    //
    // You DO need to hang on to the control instance while your program runs. It
    // does hold on to a few resources like async guards, and does manage some
    // cleanup on drop.
    //
    let control = SupportControl::load_configuartion(&cli.support)?.init();

    tracing::info!(
        config = ?cli.support.build_config(),
        cli = ?cli,
        "control initialized."
    );

    // If a local command is detected, you can handle it yourself. Otherwise, you
    // can use the support controller to execute the command provided (if any).
    //
    // Support control doesn't throw errors unless it hits a terminal state. Just
    // not executing a command is not a terminal state.
    //
    match &cli.command {
        Some(LocalCommand::Local) => {
            tracing::info!(cli = ?cli, "local command detected!");
        }
        None => {
            tracing::info!(cli = ?cli, "no local command detected!");
            tracing::info!("attempting to execute with support control");
            control
                .execute(cli.support)
                .expect("failed to execute control");
        }
    }

    Ok(())
}
