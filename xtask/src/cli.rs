use std::io::BufRead;

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
                fn run_server(name: &str, cmd: duct::Expression) {
                    let child = cmd
                        .stdout_to_stderr()
                        .stderr_to_stdout()
                        .unchecked()
                        .reader()
                        .unwrap();
                    let reader = std::io::BufReader::new(&child);

                    for line in reader.lines() {
                        println!("[{}] {}", name, line.unwrap());
                    }

                    child.try_wait().unwrap();
                }

                let dashboard = std::thread::spawn(|| {
                    run_server(
                        "dashboard",
                        duct::cmd!("cargo", "xtask", "web", "dashboard", "dev"),
                    );
                });

                let server = std::thread::spawn(|| {
                    run_server("server", duct::cmd!("cargo", "xtask", "server", "dev"));
                });

                // let dashboard = std::thread::spawn(|| web::Web::dev().run());
                // let server = std::thread::spawn(|| crate::tasks::server::dev.run());

                for thread in [dashboard, server].into_iter() {
                    thread.join().expect("Unable to join thread");
                }
            }
        }
    }
}
