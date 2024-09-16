use serde::{Deserialize, Serialize};

use super::LogLevel;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum LogLevelConfig {
    #[default]
    Undefined,
    Simple(LogLevel),
    Complex {
        min: LogLevel,
        max: LogLevel,
    },
}

impl LogLevelConfig {
    /// The range of levels that should be enabled.
    pub fn range(&self) -> std::ops::Range<LogLevel> {
        match self {
            Self::Simple(level) => *level..*level,
            Self::Complex { min, max } => *min..*max,
            Self::Undefined => LogLevel::default()..LogLevel::default(),
        }
    }

    /// If a level is lower than the minimum level, it should be enabled.
    pub fn min_level(&self) -> LogLevel {
        match self {
            Self::Simple(level) => *level,
            Self::Complex { min, .. } => *min,
            Self::Undefined => LogLevel::default(),
        }
    }

    /// If a level is lower than the maximum level, it should be disabled.
    pub fn max_level(&self) -> LogLevel {
        match self {
            Self::Simple(level) => *level,
            Self::Complex { max, .. } => *max,
            Self::Undefined => LogLevel::default(),
        }
    }
}

impl From<LogLevel> for LogLevelConfig {
    fn from(level: LogLevel) -> Self {
        Self::Simple(level)
    }
}

impl From<(LogLevel, LogLevel)> for LogLevelConfig {
    fn from(levels: (LogLevel, LogLevel)) -> Self {
        Self::Complex {
            min: levels.0,
            max: levels.1,
        }
    }
}

impl From<std::ops::Range<LogLevel>> for LogLevelConfig {
    fn from(range: std::ops::Range<LogLevel>) -> Self {
        Self::Complex {
            min: range.start,
            max: range.end,
        }
    }
}
