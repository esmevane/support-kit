#[derive(Debug, clap::Parser)]
#[clap(rename_all = "kebab-case")]
pub struct WebDashboard {
    #[clap(subcommand)]
    command: WebDashboardCommand,
}

impl crate::runnable::Runnable for WebDashboard {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, clap::Parser)]
#[clap(rename_all = "kebab-case")]
enum WebDashboardCommand {
    /// Check that the dependencies for the dashboard are ready
    Preflight,
    /// Clears the dashboard dependencies
    Clean,
    /// Installs the dashboard dependencies after checking the preflight
    Install,
    /// Runs an install, builds the web client, builds the dashboard, then
    /// copies the dashboard to the core
    Build,
}

impl crate::runnable::Runnable for WebDashboardCommand {
    fn run(&self) {
        match self {
            Self::Preflight => crate::tasks::web::dashboard::preflight.run(),
            Self::Clean => crate::tasks::web::dashboard::clean.run(),
            Self::Install => [
                crate::tasks::web::dashboard::preflight,
                crate::tasks::web::dashboard::install,
            ]
            .run(),
            Self::Build => [
                crate::tasks::web::dashboard::preflight,
                crate::tasks::web::dashboard::install,
                crate::tasks::web::client::build,
                crate::tasks::web::dashboard::build,
            ]
            .run(),
        }
    }
}
