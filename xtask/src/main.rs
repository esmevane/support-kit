use clap::Parser;
use duct::cmd;
use runnable::Runnable;

mod runnable {

    pub trait Runnable {
        fn run(&self);
    }

    impl<T> Runnable for T
    where
        T: Fn(),
    {
        fn run(&self) {
            self();
        }
    }

    impl<T> Runnable for Option<T>
    where
        T: Runnable,
    {
        fn run(&self) {
            if let Some(runnable) = self {
                runnable.run();
            }
        }
    }

    impl<T, E> Runnable for Result<T, E>
    where
        T: Runnable,
        E: std::error::Error,
    {
        fn run(&self) {
            match self {
                Ok(runnable) => runnable.run(),
                Err(error) => panic!("{}", error),
            }
        }
    }

    impl<T> Runnable for [T]
    where
        T: Runnable,
    {
        fn run(&self) {
            for runnable in self {
                runnable.run();
            }
        }
    }

    impl<T> Runnable for Vec<T>
    where
        T: Runnable,
    {
        fn run(&self) {
            for runnable in self {
                runnable.run();
            }
        }
    }
}

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

impl runnable::Runnable for Cli {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
enum Command {
    Web(Web),
}

impl runnable::Runnable for Command {
    fn run(&self) {
        match self {
            Self::Web(command) => command.run(),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
struct Web {
    #[clap(subcommand)]
    command: WebCommand,
}

impl runnable::Runnable for Web {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
enum WebCommand {
    Client(WebClient),
    Dashboard(WebDashboard),
}

impl runnable::Runnable for WebCommand {
    fn run(&self) {
        match self {
            Self::Client(action) => action.run(),
            Self::Dashboard(action) => action.run(),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
struct WebDashboard {
    #[clap(subcommand)]
    command: WebDashboardCommand,
}

impl runnable::Runnable for WebDashboard {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, Parser)]
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

impl runnable::Runnable for WebDashboardCommand {
    fn run(&self) {
        match self {
            Self::Preflight => dashboard_preflight_check.run(),
            Self::Clean => clean_dashboard.run(),
            Self::Install => [dashboard_preflight_check, install_dashboard].run(),
            Self::Build => [
                dashboard_preflight_check,
                install_dashboard,
                build_web_crate,
                build_dashboard,
            ]
            .run(),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
struct WebClient {
    #[clap(subcommand)]
    command: WebClientCommand,
}

impl runnable::Runnable for WebClient {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
enum WebClientCommand {
    Build,
}

impl runnable::Runnable for WebClientCommand {
    fn run(&self) {
        match self {
            Self::Build => build_web_crate(),
        }
    }
}

fn build_web_crate() {
    cmd!(
        "wasm-pack",
        "build",
        "--target",
        "web",
        "--out-dir",
        "../service-kit-dashboard/public/wasm",
        "service-kit-web"
    )
    .run()
    .expect("Failed to build web library");
}

/// Check that the dependencies for the dashboard are ready:
///  - pnpm
///  - node
fn dashboard_preflight_check() {
    cmd!("pnpm", "--version")
        .run()
        .expect("Failed to find pnpm");
    cmd!("node", "--version")
        .run()
        .expect("Failed to find node");
}

fn clean_dashboard() {
    cmd!("rm", "-rf", "node_modules")
        .run()
        .expect("Failed to clean dashboard");
}

fn install_dashboard() {
    cmd!("pnpm", "i",)
        .run()
        .expect("Failed to install dashboard dependencies");
}

fn build_dashboard() {
    cmd!("pnpm", "run", "--recursive", "build")
        .run()
        .expect("Failed to build dashboard");
    cmd!(
        "cp",
        "-r",
        "service-kit-dashboard/dist/",
        "service-kit-core/dist/"
    )
    .run()
    .expect("Failed to copy dashboard to core");
}

fn main() {
    Cli::parse().run();
}
