#[derive(Debug, clap::Parser)]
pub struct Container {
    #[clap(subcommand)]
    command: ContainerCommand,
}

impl crate::runnable::Runnable for Container {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, clap::Parser)]
#[clap(rename_all = "kebab-case")]
enum ContainerCommand {
    /// Check that the dependencies for the container are ready
    Preflight,
    /// Clears the container dependencies
    Clean,
    /// Installs the container dependencies after checking the preflight
    Install,
    /// Runs an install, builds the container, then copies the container to the core
    Build,
    /// Runs the container
    Run,
}

impl crate::runnable::Runnable for ContainerCommand {
    fn run(&self) {
        match self {
            Self::Preflight => crate::tasks::container::preflight(),
            Self::Clean => crate::tasks::container::clean(),
            Self::Install => crate::tasks::container::install(),
            Self::Build => crate::tasks::container::build(),
            Self::Run => crate::tasks::container::run(),
        }
    }
}
