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
                let dashboard = std::thread::spawn(|| web::Web::dev().run());
                let server = std::thread::spawn(|| crate::tasks::server::dev.run());

                dashboard.join().expect("Unable to join dashboard thread");
                server.join().expect("Unable to join server thread");
            }
        }
    }
}
