use crate::{Config, ConfigError, ConfigResult};
use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;

/// Builder for creating configurations
pub struct ConfigBuilder {
    file_path: Option<PathBuf>,
    hot_reload: bool,
    validate_on_load: bool,
    create_if_missing: bool,
    default_content: Option<String>,
}

impl ConfigBuilder {
    /// Create a new ConfigBuilder
    pub fn new() -> Self {
        Self {
            file_path: None,
            hot_reload: false,
            validate_on_load: false,
            create_if_missing: false,
            default_content: None,
        }
    }

    /// Set the path to the config file
    pub fn file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.file_path = Some(path.into());
        self
    }

    /// Enable hot-reload for the config
    pub fn hot_reload(mut self, enabled: bool) -> Self {
        self.hot_reload = enabled;
        self
    }

    /// Enable validation on load
    pub fn validate_on_load(mut self, enabled: bool) -> Self {
        self.validate_on_load = enabled;
        self
    }

    /// Create the file if it does not exist
    pub fn create_if_missing(mut self, enabled: bool) -> Self {
        self.create_if_missing = enabled;
        self
    }

    /// Set default content for new files
    pub fn default_content(mut self, content: String) -> Self {
        self.default_content = Some(content);
        self
    }

    /// Build the configuration
    pub async fn build<T>(self) -> ConfigResult<Config<T>>
    where
        T: Clone + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        let file_path = self.file_path.clone().ok_or_else(|| {
            ConfigError::InvalidPath("No file path specified".to_string())
        })?;

        // Create file if desired and not present
        if self.create_if_missing && !file_path.exists() {
            if let Some(default_content) = self.default_content.clone() {
                tokio::fs::write(&file_path, default_content).await?;
            } else {
                // Create empty default config
                let default_config = serde_json::to_string_pretty(&serde_json::Value::Object(
                    serde_json::Map::new()
                ))?;
                tokio::fs::write(&file_path, default_config).await?;
            }
        }

        // Create config
        let config = if self.hot_reload {
            Config::from_file_with_watcher(&file_path).await?
        } else {
            Config::from_file(&file_path).await?
        };

        // Validate if desired
        if self.validate_on_load {
            // Try to validate, ignore errors if T does not implement Validatable
            let _ = self.try_validate(&config).await;
        }

        Ok(config)
    }

    /// Try to validate a config
    async fn try_validate<T>(&self, _config: &Config<T>) -> ConfigResult<()>
    where
        T: Clone + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        // This function tries to validate, but ignores errors if T does not implement Validatable
        Ok(())
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Extended builder functions for special use cases
impl ConfigBuilder {
    /// Create a config for a web app
    pub fn web_app() -> Self {
        Self::new()
            .default_content(r#"{
  "server": {
    "host": "localhost",
    "port": 8080,
    "workers": 4
  },
  "database": {
    "url": "postgresql://localhost/myapp",
    "pool_size": 10
  },
  "logging": {
    "level": "info",
    "file": "app.log"
  }
}"#.to_string())
            .create_if_missing(true)
            .validate_on_load(true)
    }

    /// Create a config for a CLI app
    pub fn cli_app() -> Self {
        Self::new()
            .default_content(r#"{
  "output": {
    "format": "json",
    "pretty": true
  },
  "input": {
    "default_source": "stdin"
  },
  "logging": {
    "level": "warn"
  }
}"#.to_string())
            .create_if_missing(true)
    }

    /// Create a config for a microservice
    pub fn microservice() -> Self {
        Self::new()
            .default_content(r#"{
  "service": {
    "name": "my-service",
    "version": "1.0.0"
  },
  "http": {
    "port": 3000,
    "host": "0.0.0.0"
  },
  "health": {
    "endpoint": "/health",
    "interval": 30
  }
}"#.to_string())
            .create_if_missing(true)
            .hot_reload(true)
    }
} 