use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serde error: {0}")]
    Serde(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid file format: {0}")]
    InvalidFormat(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Hot-reload error: {0}")]
    HotReload(String),

    #[error("Configuration not initialized")]
    NotInitialized,

    #[error("Format not supported: {0}")]
    FormatNotSupported(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Timeout while loading configuration")]
    Timeout,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for configuration operations
pub type ConfigResult<T> = Result<T, ConfigError>;

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::Serde(format!("JSON: {}", err))
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        ConfigError::Serde(format!("YAML: {}", err))
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::Serde(format!("TOML: {}", err))
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(err: toml::ser::Error) -> Self {
        ConfigError::Serde(format!("TOML Serialization: {}", err))
    }
}

#[cfg(feature = "hot-reload")]
impl From<notify::Error> for ConfigError {
    fn from(err: notify::Error) -> Self {
        ConfigError::HotReload(err.to_string())
    }
}
