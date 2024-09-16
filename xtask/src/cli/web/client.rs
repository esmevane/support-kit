#[derive(Debug, clap::Parser)]
#[clap(rename_all = "kebab-case")]
pub struct WebClient {
    #[clap(subcommand)]
    command: WebClientCommand,
}

impl crate::runnable::Runnable for WebClient {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, clap::Parser)]
#[clap(rename_all = "kebab-case")]
enum WebClientCommand {
    Build,
}

impl crate::runnable::Runnable for WebClientCommand {
    fn run(&self) {
        match self {
            Self::Build => crate::tasks::web::client::build(),
        }
    }
}
