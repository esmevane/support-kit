mod args;
mod config;
mod logs;
mod network;
mod structures;
mod verbosity_level;

pub use args::Args;
pub use config::Config;
pub use logs::*;
pub use structures::*;
pub use verbosity_level::VerbosityLevel;

type TracingTarget = Box<dyn tracing_subscriber::Layer<tracing_subscriber::Registry> + Send + Sync>;
type TracingTargets = Vec<TracingTarget>;

pub use network::NetworkConfig;

#[test]
fn todos() {
    let todos = include_str!("../todo.md");

    assert!(false, "{todos}");
}
