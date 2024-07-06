use crate::{settings::Settings, APP_NAME};
use clap_verbosity_flag::Verbosity;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt::time::ChronoLocal, layer::SubscriberExt};

fn calculate_env_filter(verbosity: &Verbosity) -> String {
    let log_level = verbosity.log_level_filter().as_str();

    format!("{log_level},{APP_NAME}_support={log_level},tower_http=debug,axum::rejection=trace")
}

pub fn init(settings: &Settings) -> WorkerGuard {
    let env_filter_config = calculate_env_filter(&settings.cli.global.verbosity);

    let directory = "logs";
    let file_name_prefix = "monitor.log";

    let appender = tracing_appender::rolling::daily(directory, file_name_prefix);
    let (non_blocking, guard) = tracing_appender::non_blocking(appender);
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| env_filter_config.clone().into());

    let log_subscriber = tracing_subscriber::fmt::Layer::new().with_writer(std::io::stdout);
    let stdout_subscriber = tracing_subscriber::fmt::Layer::new()
        .json()
        .with_writer(non_blocking)
        .with_timer(ChronoLocal::default());

    let collector = tracing_subscriber::registry()
        .with(env_filter)
        .with(log_subscriber)
        .with(stdout_subscriber);

    tracing::subscriber::set_global_default(collector).expect("setting default subscriber failed");

    tracing::info!(
        app_name = APP_NAME,
        "Logging initialized: {env_filter_config}",
    );

    guard
}
