pub mod client;
pub mod dashboard;

#[derive(Debug, clap::Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Web {
    #[clap(subcommand)]
    command: WebCommand,
}

impl Web {
    pub fn dev() -> Self {
        Self {
            command: WebCommand::Dashboard(dashboard::WebDashboard::dev()),
        }
    }
}

impl crate::runnable::Runnable for Web {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, clap::Parser)]
#[clap(rename_all = "kebab-case")]
enum WebCommand {
    /// Work with the wasm web client library
    Client(client::WebClient),
    /// Manage the web dashboard: preflight, clean, install, build, etc.
    Dashboard(dashboard::WebDashboard),
}

impl crate::runnable::Runnable for WebCommand {
    fn run(&self) {
        match self {
            Self::Client(action) => action.run(),
            Self::Dashboard(action) => action.run(),
        }
    }
}
