#[derive(Debug, clap::Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Server {
    #[clap(subcommand)]
    command: Command,
}

impl Server {
    pub fn dev() -> Self {
        Self {
            command: Command::Dev,
        }
    }
}

impl crate::runnable::Runnable for Server {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, clap::Parser)]
#[clap(rename_all = "kebab-case")]
enum Command {
    /// Run the server in development mode.
    Dev,
}

impl crate::runnable::Runnable for Command {
    fn run(&self) {
        match self {
            Self::Dev => crate::tasks::server::dev.run(),
        }
    }
}
