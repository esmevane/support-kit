use crate::{
    settings::{MinMax, Settings, VerbosityDefinition},
    APP_NAME,
};
use clap_verbosity_flag::Verbosity;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{time::ChronoLocal, writer::MakeWriterExt},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

fn calculate_env_filter(verbosity: &Verbosity) -> String {
    let log_level = verbosity.log_level_filter().as_str();

    format!("{log_level},{APP_NAME}_support={log_level},tower_http=debug,axum::rejection=trace")
}

pub fn init(settings: &Settings) -> Vec<WorkerGuard> {
    let env_filter_config = calculate_env_filter(&settings.cli.global.verbosity);
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| env_filter_config.clone().into());

    let mut min_max_layers = Vec::new();
    let mut simple_layers = Vec::new();
    let mut guards = Vec::new();

    for logger in settings.config.logging.loggers.clone() {
        match logger.level() {
            VerbosityDefinition::MinMax(MinMax { min, max }) => {
                let appender = logger.appender();
                let (non_blocking, guard) = tracing_appender::non_blocking(appender);
                let layer = tracing_subscriber::fmt::layer()
                    .json()
                    .with_writer(
                        non_blocking
                            .with_max_level(max.as_level_filter())
                            .with_min_level(min.as_level_filter()),
                    )
                    .with_timer(ChronoLocal::default());

                min_max_layers.push(layer);
                guards.push(guard);
            }
            VerbosityDefinition::Single(level) => {
                let appender = logger.appender();
                let (non_blocking, guard) = tracing_appender::non_blocking(appender);
                let layer = tracing_subscriber::fmt::layer()
                    .json()
                    .with_writer(non_blocking)
                    .with_timer(ChronoLocal::default())
                    .with_filter(
                        tracing_subscriber::EnvFilter::from_default_env()
                            .add_directive(level.as_level_filter().into()),
                    );

                simple_layers.push(layer);
                guards.push(guard);
            }
        }
    }

    // let (non_blocking, guard) = tracing_appender::non_blocking();
    // let logging_appender = tracing_subscriber::fmt::Layer::new()
    //     .with_writer(non_blocking)
    //     .with_timer(ChronoLocal::default())
    //     .json();
    // let subscriber = tracing_subscriber::registry().with(env_filter);
    //     subscriber = subscriber.with(logging_appender);

    // let directory = "logs";
    // let file_name_prefix = "monitor.log";

    // let appender = tracing_appender::rolling::daily(directory, file_name_prefix);
    // let (non_blocking, guard) = tracing_appender::non_blocking(appender);

    // let log_subscriber = tracing_subscriber::fmt::Layer::new()
    //     .json()
    //     .with_writer(non_blocking)
    //     .with_timer(ChronoLocal::default());

    tracing_subscriber::registry()
        .with(env_filter)
        // .with(tracing_subscriber::fmt::layer())
        // .with(log_subscriber)
        .with(simple_layers)
        .with(min_max_layers)
        .init();

    tracing::info!(
        app_name = APP_NAME,
        "Logging initialized: {env_filter_config}",
    );

    guards
}
