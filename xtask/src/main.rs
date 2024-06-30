use clap::Parser;
use duct::cmd;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

impl Cli {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
enum Command {
    Web(WebAction),
}

impl Command {
    fn run(&self) {
        match self {
            Command::Web(web_action) => web_action.run(),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
struct WebAction {
    #[clap(subcommand)]
    command: WebActionCommand,
}

impl WebAction {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
enum WebActionCommand {
    Build,
}

impl WebActionCommand {
    fn run(&self) {
        match self {
            Self::Build => {
                cmd!(
                    "wasm-pack",
                    "build",
                    "--target",
                    "web",
                    "--out-dir",
                    "../service-kit-core/dist/wasm",
                    "service-kit-web"
                )
                .run()
                .expect("Failed to build web library");
            }
        }
    }
}

fn main() {
    Cli::parse().run();
}
