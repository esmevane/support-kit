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

    println!(
        "CLI: {cli:?}\nCONFIG: {config:?}",
        config = cli.support.config(),
        cli = &cli,
    );

    match &cli.command {
        Some(LocalCommand::Local) => {
            println!("Local command detected! {cli:?}");
        }
        None => {
            println!("No local command detected! {cli:?}");
        }
    }
}
