mod container;
mod web;

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

impl crate::runnable::Runnable for Cli {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, clap::Parser)]
#[clap(rename_all = "kebab-case")]
enum Command {
    /// Work with the container
    Container(container::Container),
    /// Work with the web client and dashboard
    Web(web::Web),
}

impl crate::runnable::Runnable for Command {
    fn run(&self) {
        match self {
            Self::Container(command) => command.run(),
            Self::Web(command) => command.run(),
        }
    }
}
