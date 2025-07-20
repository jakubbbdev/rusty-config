use crate::{ConfigError, ConfigResult};
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

/// Load a config from a file
pub async fn load_from_file<T>(path: &Path) -> ConfigResult<T>
where
    T: DeserializeOwned,
{
    if !path.exists() {
        return Err(ConfigError::FileNotFound(
            path.to_string_lossy().to_string(),
        ));
    }

    let content = tokio::fs::read_to_string(path).await?;
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "json" => load_json(&content),
        "yaml" | "yml" => load_yaml(&content),
        "toml" => load_toml(&content),
        _ => {
            // Try to auto-detect based on content
            if content.trim().starts_with('{') || content.trim().starts_with('[') {
                load_json(&content)
            } else if content.trim().starts_with('#') || content.contains(':') {
                load_yaml(&content)
            } else {
                load_toml(&content)
            }
        }
    }
}

/// Save a config to a file
pub async fn save_to_file<T>(path: &Path, data: &T) -> ConfigResult<()>
where
    T: Serialize,
{
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    let content = match extension.as_str() {
        "json" => save_json(data)?,
        "yaml" | "yml" => save_yaml(data)?,
        "toml" => save_toml(data)?,
        _ => save_json(data)?, // Default is JSON
    };

    tokio::fs::write(path, content).await?;
    Ok(())
}

/// Load JSON config
#[cfg(feature = "json")]
fn load_json<T>(content: &str) -> ConfigResult<T>
where
    T: DeserializeOwned,
{
    serde_json::from_str(content).map_err(ConfigError::from)
}

#[cfg(not(feature = "json"))]
fn load_json<T>(_content: &str) -> ConfigResult<T>
where
    T: DeserializeOwned,
{
    Err(ConfigError::FormatNotSupported("JSON".to_string()))
}

/// Save JSON config
#[cfg(feature = "json")]
fn save_json<T>(data: &T) -> ConfigResult<String>
where
    T: Serialize,
{
    serde_json::to_string_pretty(data).map_err(ConfigError::from)
}

#[cfg(not(feature = "json"))]
fn save_json<T>(_data: &T) -> ConfigResult<String>
where
    T: Serialize,
{
    Err(ConfigError::FormatNotSupported("JSON".to_string()))
}

/// Load YAML config
#[cfg(feature = "yaml")]
fn load_yaml<T>(content: &str) -> ConfigResult<T>
where
    T: DeserializeOwned,
{
    serde_yaml::from_str(content).map_err(ConfigError::from)
}

#[cfg(not(feature = "yaml"))]
fn load_yaml<T>(_content: &str) -> ConfigResult<T>
where
    T: DeserializeOwned,
{
    Err(ConfigError::FormatNotSupported("YAML".to_string()))
}

/// Save YAML config
#[cfg(feature = "yaml")]
fn save_yaml<T>(data: &T) -> ConfigResult<String>
where
    T: Serialize,
{
    serde_yaml::to_string(data).map_err(ConfigError::from)
}

#[cfg(not(feature = "yaml"))]
fn save_yaml<T>(_data: &T) -> ConfigResult<String>
where
    T: Serialize,
{
    Err(ConfigError::FormatNotSupported("YAML".to_string()))
}

/// Load TOML config
#[cfg(feature = "toml")]
fn load_toml<T>(content: &str) -> ConfigResult<T>
where
    T: DeserializeOwned,
{
    toml::from_str(content).map_err(ConfigError::from)
}

#[cfg(not(feature = "toml"))]
fn load_toml<T>(_content: &str) -> ConfigResult<T>
where
    T: DeserializeOwned,
{
    Err(ConfigError::FormatNotSupported("TOML".to_string()))
}

/// Save TOML config
#[cfg(feature = "toml")]
fn save_toml<T>(data: &T) -> ConfigResult<String>
where
    T: Serialize,
{
    toml::to_string_pretty(data).map_err(ConfigError::from)
}

#[cfg(not(feature = "toml"))]
fn save_toml<T>(_data: &T) -> ConfigResult<String>
where
    T: Serialize,
{
    Err(ConfigError::FormatNotSupported("TOML".to_string()))
}

/// Detect the format of a file based on its extension
pub fn detect_format(path: &Path) -> Option<ConfigFormat> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| match ext.to_lowercase().as_str() {
            "json" => ConfigFormat::Json,
            "yaml" | "yml" => ConfigFormat::Yaml,
            "toml" => ConfigFormat::Toml,
            _ => ConfigFormat::Unknown,
        })
}

/// Supported config formats
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
    Unknown,
}

impl ConfigFormat {
    /// Get the default file extension for the format
    pub fn extension(&self) -> &'static str {
        match self {
            ConfigFormat::Json => "json",
            ConfigFormat::Yaml => "yaml",
            ConfigFormat::Toml => "toml",
            ConfigFormat::Unknown => "json",
        }
    }

    /// Get the MIME type for the format
    pub fn mime_type(&self) -> &'static str {
        match self {
            ConfigFormat::Json => "application/json",
            ConfigFormat::Yaml => "application/x-yaml",
            ConfigFormat::Toml => "application/toml",
            ConfigFormat::Unknown => "application/json",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestConfig {
        name: String,
        value: i32,
        enabled: bool,
    }

    #[tokio::test]
    async fn test_load_save_json() {
        #[cfg(feature = "json")]
        {
            let config = TestConfig {
                name: "test".to_string(),
                value: 42,
                enabled: true,
            };

            let temp_file = tempfile::NamedTempFile::new().unwrap();
            let path = temp_file.path().with_extension("json");

            // Save
            save_to_file(&path, &config).await.unwrap();

            // Load
            let loaded: TestConfig = load_from_file(&path).await.unwrap();
            assert_eq!(config, loaded);
        }
    }

    #[tokio::test]
    async fn test_load_save_yaml() {
        #[cfg(feature = "yaml")]
        {
            let config = TestConfig {
                name: "test".to_string(),
                value: 42,
                enabled: true,
            };

            let temp_file = tempfile::NamedTempFile::new().unwrap();
            let path = temp_file.path().with_extension("yaml");

            // Save
            save_to_file(&path, &config).await.unwrap();

            // Load
            let loaded: TestConfig = load_from_file(&path).await.unwrap();
            assert_eq!(config, loaded);
        }
    }

    #[tokio::test]
    async fn test_load_save_toml() {
        #[cfg(feature = "toml")]
        {
            let config = TestConfig {
                name: "test".to_string(),
                value: 42,
                enabled: true,
            };

            let temp_file = tempfile::NamedTempFile::new().unwrap();
            let path = temp_file.path().with_extension("toml");

            // Save
            save_to_file(&path, &config).await.unwrap();

            // Load
            let loaded: TestConfig = load_from_file(&path).await.unwrap();
            assert_eq!(config, loaded);
        }
    }

    #[test]
    fn test_detect_format() {
        assert_eq!(
            detect_format(Path::new("config.json")),
            Some(ConfigFormat::Json)
        );
        assert_eq!(
            detect_format(Path::new("config.yaml")),
            Some(ConfigFormat::Yaml)
        );
        assert_eq!(
            detect_format(Path::new("config.yml")),
            Some(ConfigFormat::Yaml)
        );
        assert_eq!(
            detect_format(Path::new("config.toml")),
            Some(ConfigFormat::Toml)
        );
        assert_eq!(
            detect_format(Path::new("config.txt")),
            Some(ConfigFormat::Unknown)
        );
    }
}
