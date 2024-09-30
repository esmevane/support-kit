mod config;
mod logs;
mod structures;

pub use config::Config;
pub use logs::*;
pub use structures::*;

type TracingTarget = Box<dyn tracing_subscriber::Layer<tracing_subscriber::Registry> + Send + Sync>;
type TracingTargets = Vec<TracingTarget>;
