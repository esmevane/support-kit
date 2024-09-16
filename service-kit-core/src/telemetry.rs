use clap_verbosity_flag::Verbosity;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{time::ChronoLocal, writer::MakeWriterExt},
    layer::SubscriberExt,
    Layer,
};

use crate::{settings::Settings, APP_NAME};

fn calculate_env_filter(verbosity: &Verbosity) -> String {
    let log_level = verbosity.log_level_filter().as_str();

    format!("{log_level},{APP_NAME}_support={log_level},tower_http=debug,axum::rejection=trace")
}

pub fn init(settings: &Settings) -> Vec<WorkerGuard> {
    let env_filter_config = calculate_env_filter(&settings.cli.global.verbosity);
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| env_filter_config.clone().into());

    let mut layers = Vec::new();
    let mut guards = Vec::new();

    for logger in settings.loggers() {
        let appender = logger.appender();
        let (non_blocking, guard) = tracing_appender::non_blocking(appender);

        let logger = tracing_subscriber::fmt::layer()
            .json()
            .with_writer(
                non_blocking
                    .with_min_level(logger.min_level())
                    .with_max_level(logger.max_level()),
            )
            .with_timer(ChronoLocal::default());

        layers.push(logger);
        guards.push(guard);
    }

    let stdout = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout.with_min_level(tracing::Level::INFO))
        .with_filter(env_filter);

    let stderr = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr.with_max_level(tracing::Level::WARN));

    let subscriber = tracing_subscriber::registry()
        .with(stdout)
        .with(stderr)
        .with(layers);

    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");

    tracing::info!(
        app_name = APP_NAME,
        "Logging initialized: {env_filter_config}",
    );

    tracing::error!("Error log level enabled");
    tracing::warn!("Warn log level enabled");
    tracing::info!("Info log level enabled");
    tracing::debug!("Debug log level enabled");
    tracing::trace!("Trace log level enabled");

    guards
}
