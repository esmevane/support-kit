use crate::APP_NAME;
use clap_verbosity_flag::Verbosity;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

static DEFAULT_ENV_FILTER: &str = "tower_http=debug,axum::rejection=trace";

pub fn init(verbosity: &Verbosity) {
    let log_level = verbosity.log_level_filter().as_str();
    let default_env_filter =
        format!("{log_level},{APP_NAME}_support={log_level},{DEFAULT_ENV_FILTER}");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| default_env_filter.clone().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!(
        app_name = APP_NAME,
        "Logging initialized: {default_env_filter}",
    );
}
