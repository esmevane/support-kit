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
            Command::Web(command) => command.run(),
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
    Client(WebClient),
    Dashboard(WebDashboard),
}

impl WebCommand {
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

impl WebDashboard {
    fn run(&self) {
        self.command.run();
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
enum WebDashboardCommand {
    Preflight,
    Clean,
    Install,
    Build,
}

impl WebDashboardCommand {
    fn run(&self) {
        match self {
            Self::Preflight => dashboard_preflight_check(),
            Self::Clean => clean_dashboard(),
            Self::Install => install_dashboard(),
            Self::Build => build_dashboard(),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
struct WebClient {
    #[clap(subcommand)]
    command: WebCrateCommand,
}

impl WebClient {
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

fn clean_dashboard() {
    cmd!("rm", "-rf", "node_modules")
        .run()
        .expect("Failed to clean dashboard");
}

fn install_dashboard() {
    dashboard_preflight_check();
    cmd!("pnpm", "i",)
        .run()
        .expect("Failed to install dashboard dependencies");
}

fn build_dashboard() {
    install_dashboard();
    build_web_crate();
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
