use crate::{settings::Settings, APP_NAME};
use clap_verbosity_flag::Verbosity;
use tracing_subscriber::fmt::time::ChronoLocal;

fn calculate_env_filter(verbosity: &Verbosity) -> String {
    let log_level = verbosity.log_level_filter().as_str();

    format!("{log_level},{APP_NAME}_support={log_level},tower_http=debug,axum::rejection=trace")
}

pub fn init(settings: &Settings) {
    let env_filter_config = calculate_env_filter(&settings.cli.global.verbosity);

    // Setup tracing with rotating log files
    let file_appender = tracing_appender::rolling::daily("logs", "monitor.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| env_filter_config.clone().into());

    tracing_subscriber::fmt()
        .json()
        .with_writer(non_blocking)
        .with_timer(ChronoLocal::default())
        .with_env_filter(env_filter)
        .init();

    tracing::info!(
        app_name = APP_NAME,
        "Logging initialized: {env_filter_config}",
    );
}
