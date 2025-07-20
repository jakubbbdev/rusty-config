//! # RustyConfig
//!
//! A smart configuration management library for Rust with hot-reload, validation, and multiple backend formats.
//!
//! ## Features
//!
//! - 🔄 **Hot-Reload**: Automatic reloading of configuration files
//! - ✅ **Validation**: Robust validation of configuration data
//! - 📁 **Multi-Format**: Support for YAML, JSON, and TOML
//! - 🚀 **Async**: Fully asynchronous API
//! - 🔒 **Type-Safe**: Strongly typed configurations
//!
//! ## Example
//!
//! ```rust
//! use rusty_config::{Config, ConfigBuilder};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! struct AppConfig {
//!     server: ServerConfig,
//!     database: DatabaseConfig,
//! }
//!
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! struct ServerConfig {
//!     host: String,
//!     port: u16,
//! }
//!
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! struct DatabaseConfig {
//!     url: String,
//!     pool_size: u32,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ConfigBuilder::new()
//!         .file("config.yaml")
//!         .create_if_missing(true)
//!         .default_content(r#"{
//!   "server": {
//!     "host": "localhost",
//!     "port": 8080
//!   },
//!   "database": {
//!     "url": "postgres://localhost/db",
//!     "pool_size": 10
//!   }
//! }"#.to_string())
//!         .build::<AppConfig>()
//!         .await?;
//!
//!     let app_config = config.get();
//!     println!("Server running on {}:{}", app_config.server.host, app_config.server.port);
//!     Ok(())
//! }
//! ```

pub mod builder;
pub mod config;
pub mod error;
pub mod loader;
pub mod validator;
pub mod watcher;

pub use builder::ConfigBuilder;
pub use config::Config;
pub use error::{ConfigError, ConfigResult};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{Config, ConfigBuilder, ConfigError, ConfigResult};
    pub use async_trait::async_trait;
    pub use serde::{Deserialize, Serialize};
}
