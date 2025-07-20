use crate::{ConfigError, ConfigResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait for validatable configurations
#[async_trait]
pub trait Validatable {
    /// Validate the configuration
    async fn validate(&self) -> ConfigResult<()>;
}

/// Standard validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub code: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            code: "VALIDATION_ERROR".to_string(),
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = code.into();
        self
    }
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.is_valid = false;
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: ValidationError) {
        self.warnings.push(warning);
    }

    pub fn merge(&mut self, other: ValidationResult) {
        self.is_valid = self.is_valid && other.is_valid;
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate a configuration
pub async fn validate<T>(config: &T) -> ConfigResult<()>
where
    T: Validatable,
{
    config.validate().await
}

/// Validate a configuration and return detailed results
pub async fn validate_detailed<T>(config: &T) -> ConfigResult<ValidationResult>
where
    T: Validatable + DetailedValidatable,
{
    config.validate_detailed().await
}

/// Extended validation with detailed results
#[async_trait]
pub trait DetailedValidatable: Validatable {
    async fn validate_detailed(&self) -> ConfigResult<ValidationResult>;
}

/// Validation rules
pub struct ValidationRule<T> {
    pub name: String,
    pub validator: Box<dyn Fn(&T) -> ConfigResult<()> + Send + Sync>,
}

impl<T> ValidationRule<T> {
    pub fn new<F>(name: impl Into<String>, validator: F) -> Self
    where
        F: Fn(&T) -> ConfigResult<()> + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            validator: Box::new(validator),
        }
    }

    pub fn validate(&self, value: &T) -> ConfigResult<()> {
        (self.validator)(value)
    }
}

/// Validator for various data types
pub struct TypeValidator;

impl TypeValidator {
    /// Validate that a string is not empty
    pub fn not_empty(value: &str, field_name: &str) -> ConfigResult<()> {
        if value.trim().is_empty() {
            return Err(ConfigError::Validation(format!(
                "Field '{}' must not be empty",
                field_name
            )));
        }
        Ok(())
    }

    /// Validate that a string has a certain length
    pub fn length(value: &str, min: usize, max: usize, field_name: &str) -> ConfigResult<()> {
        let len = value.len();
        if len < min || len > max {
            return Err(ConfigError::Validation(format!(
                "Field '{}' must be between {} and {} characters long (currently: {})",
                field_name, min, max, len
            )));
        }
        Ok(())
    }

    /// Validate that a number is within a certain range
    pub fn range<T>(value: T, min: T, max: T, field_name: &str) -> ConfigResult<()>
    where
        T: PartialOrd + std::fmt::Display,
    {
        if value < min || value > max {
            return Err(ConfigError::Validation(format!(
                "Field '{}' must be between {} and {} (currently: {})",
                field_name, min, max, value
            )));
        }
        Ok(())
    }

    /// Validate a URL
    pub fn url(value: &str, field_name: &str) -> ConfigResult<()> {
        if !value.starts_with("http://") && !value.starts_with("https://") {
            return Err(ConfigError::Validation(format!(
                "Field '{}' must be a valid URL (start with http:// or https://)",
                field_name
            )));
        }
        Ok(())
    }

    /// Validate an email address (simple validation)
    pub fn email(value: &str, field_name: &str) -> ConfigResult<()> {
        if !value.contains('@') || !value.contains('.') {
            return Err(ConfigError::Validation(format!(
                "Field '{}' must be a valid email address",
                field_name
            )));
        }
        Ok(())
    }

    /// Validate that a port is in the valid range
    pub fn port(value: u16, field_name: &str) -> ConfigResult<()> {
        if value == 0 || value > 65535 {
            return Err(ConfigError::Validation(format!(
                "Field '{}' must be a valid port between 1 and 65535 (currently: {})",
                field_name, value
            )));
        }
        Ok(())
    }
}

/// Predefined validation rules for common use cases
pub struct CommonValidators;

impl CommonValidators {
    /// Validate a server config
    pub fn validate_server_config(host: &str, port: u16) -> ConfigResult<()> {
        TypeValidator::not_empty(host, "host")?;
        TypeValidator::port(port, "port")?;
        Ok(())
    }

    /// Validate a database config
    pub fn validate_database_config(url: &str, pool_size: u32) -> ConfigResult<()> {
        TypeValidator::not_empty(url, "database_url")?;
        TypeValidator::range(pool_size, 1, 100, "pool_size")?;
        Ok(())
    }

    /// Validate a logging config
    pub fn validate_logging_config(level: &str) -> ConfigResult<()> {
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&level.to_lowercase().as_str()) {
            return Err(ConfigError::Validation(format!(
                "Logging level '{}' is invalid. Valid values: {:?}",
                level, valid_levels
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestConfig {
        name: String,
        port: u16,
        email: String,
        url: String,
    }

    #[async_trait]
    impl Validatable for TestConfig {
        async fn validate(&self) -> ConfigResult<()> {
            TypeValidator::not_empty(&self.name, "name")?;
            TypeValidator::port(self.port, "port")?;
            TypeValidator::email(&self.email, "email")?;
            TypeValidator::url(&self.url, "url")?;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_valid_config() {
        let config = TestConfig {
            name: "test".to_string(),
            port: 8080,
            email: "test@example.com".to_string(),
            url: "https://example.com".to_string(),
        };

        assert!(config.validate().await.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_config() {
        let config = TestConfig {
            name: "".to_string(),
            port: 0,
            email: "invalid-email".to_string(),
            url: "not-a-url".to_string(),
        };

        assert!(config.validate().await.is_err());
    }

    #[test]
    fn test_type_validators() {
        assert!(TypeValidator::not_empty("test", "field").is_ok());
        assert!(TypeValidator::not_empty("", "field").is_err());

        assert!(TypeValidator::length("test", 1, 10, "field").is_ok());
        assert!(TypeValidator::length("", 1, 10, "field").is_err());

        assert!(TypeValidator::range(5, 1, 10, "field").is_ok());
        assert!(TypeValidator::range(0, 1, 10, "field").is_err());

        assert!(TypeValidator::url("https://example.com", "field").is_ok());
        assert!(TypeValidator::url("not-a-url", "field").is_err());

        assert!(TypeValidator::email("test@example.com", "field").is_ok());
        assert!(TypeValidator::email("invalid-email", "field").is_err());

        assert!(TypeValidator::port(8080, "field").is_ok());
        assert!(TypeValidator::port(0, "field").is_err());
    }
} 