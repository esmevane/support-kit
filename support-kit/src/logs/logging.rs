use tracing_appender::non_blocking::WorkerGuard;

use crate::{Configuration, TracingTargets};

use super::LoggingConfig;

#[derive(Default)]
pub struct Logging {
    config: LoggingConfig,
    pub loggers: TracingTargets,
    pub guards: Vec<WorkerGuard>,
}

impl Logging {
    pub fn initialize(config: Configuration) -> Vec<WorkerGuard> {
        use tracing_subscriber::layer::SubscriberExt;

        let mut logging = Self::default();

        for logger in config.loggers() {
            logger.initialize(&config, &mut logging)
        }

        let subscriber = tracing_subscriber::registry().with(logging.loggers);

        tracing::subscriber::set_global_default(subscriber)
            .expect("Unable to set a global subscriber");

        logging.guards
    }
}

impl std::fmt::Debug for Logging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Logging")
            .field("config", &self.config)
            .field("loggers", &self.loggers.len())
            .field("guards", &self.guards)
            .finish()
    }
}

impl PartialEq for Logging {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
    }
}
