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
    Web(Web),
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
struct Web {
    #[clap(subcommand)]
    command: WebCommand,
}

impl Web {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
enum WebCommand {
    Crate(WebCrateAction),
    Dashboard(WebDashboardAction),
}

impl WebCommand {
    fn run(&self) {
        match self {
            Self::Crate(action) => action.run(),
            Self::Dashboard(action) => action.run(),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
struct WebDashboardAction {
    #[clap(subcommand)]
    command: WebDashboardCommand,
}

impl WebDashboardAction {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
enum WebDashboardCommand {
    Check,
    // Clean,
    Install,
    // Build,
}

impl WebDashboardCommand {
    fn run(&self) {
        match self {
            Self::Check => check_dashboard(),
            // Self::Clean => clean_dashboard(),
            Self::Install => install_dashboard(),
            // Self::Build => build_dashboard(),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
struct WebCrateAction {
    #[clap(subcommand)]
    command: WebCrateCommand,
}

impl WebCrateAction {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
enum WebCrateCommand {
    Build,
}

impl WebCrateCommand {
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
        "../service-kit-core/dist/wasm",
        "service-kit-web"
    )
    .run()
    .expect("Failed to build web library");
}

/// Check that the dependencies for the dashboard are ready:
///  - pnpm
///  - node
fn check_dashboard() {
    let pnpm_version = cmd!("pnpm", "--version").run();
    let node_version = cmd!("node", "--version").run();

    match (pnpm_version, node_version) {
        (Ok(pnpm_version), Ok(node_version)) => {
            tracing::info!(
                pnpm = format!("pnpm: {pnpm_version:?}"),
                node = format!("node: {node_version:?}"),
            );
        }
        (Err(pnpm_version), Ok(node_version)) => {
            tracing::error!(
                node = ?node_version,
                pnpm = format!("pnpm not found: {pnpm_version}, install with `npm install -g pnpm`"),
            );
        }
        (Ok(pnpm_version), Err(node_version)) => {
            tracing::error!(
                pnpm = ?pnpm_version,
                node = format!("node not found: {node_version}, install directions on https://nodejs.org/en/"),
            );
        }
        (Err(pnpm_version), Err(node_version)) => {
            tracing::error!(
                pnpm =
                    format!("pnpm not found: {pnpm_version}, install with `npm install -g pnpm`"),
                node = format!(
                    "node not found: {node_version}, install directions on https://nodejs.org/en/"
                ),
            );
        }
    }
}

fn install_dashboard() {
    cmd!(
        "pnpm",
        "install",
        "--prefix",
        "../service-kit-core/dashboard"
    )
    .run()
    .expect("Failed to install dashboard dependencies");
}

fn main() {
    Cli::parse().run();
}
