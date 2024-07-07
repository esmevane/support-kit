mod container;
mod server;
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
    /// Work with the server
    Server(server::Server),
    /// Run the project in development mode
    Dev,
}

impl crate::runnable::Runnable for Command {
    fn run(&self) {
        match self {
            Self::Container(command) => command.run(),
            Self::Web(command) => command.run(),
            Self::Server(command) => command.run(),
            Self::Dev => {
                let dashboard = web::Web::dev().background();
                let server = server::Server::dev().background();

                for thread in [dashboard, server].into_iter() {
                    thread.join().expect("Unable to join thread");
                }
            }
        }
    }
}
