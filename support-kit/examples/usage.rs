use support_kit::SupportControl;

pub fn main() {
    use clap::{Parser, Subcommand};

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

    let cli = YourOwnCli::parse();
    let control = SupportControl::from_args(&cli.support).init();

    tracing::info!(
        config = ?cli.support.config(),
        cli = ?cli,
        "Control initialized."
    );

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
}
