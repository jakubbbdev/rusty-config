# 🦀 RustyConfig

[![Crates.io](https://img.shields.io/crates/v/rusty-config)](https://crates.io/crates/rusty-config)
[![Documentation](https://docs.rs/rusty-config/badge.svg)](https://docs.rs/rusty-config)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

A smart configuration management library for Rust with hot-reload, validation, and multiple backend formats.

## ✨ Features

- 🔄 **Hot-Reload**: Automatic reloading of configuration files
- ✅ **Validation**: Robust validation of configuration data
- 📁 **Multi-Format**: Support for YAML, JSON, and TOML
- 🚀 **Async**: Fully asynchronous API
- 🔒 **Type-Safe**: Strongly typed configurations
- 🛠️ **Builder Pattern**: User-friendly API
- 📊 **Versioning**: Track configuration changes
- 🔍 **Error Handling**: Detailed error messages

## 🚀 Installation

Add `rusty-config` to your `Cargo.toml` dependencies:

```toml
[dependencies]
rusty-config = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

## 📖 Quick Start

### Basic Usage

```rust
use rusty_config::{Config, ConfigBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DatabaseConfig {
    url: String,
    pool_size: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a config
    let config = ConfigBuilder::new()
        .file("config.yaml")
        .hot_reload(true)
        .build::<AppConfig>()
        .await?;

    // Use the config
    let app_config = config.get();
    println!("Server running on {}:{}", app_config.server.host, app_config.server.port);
    
    Ok(())
}
```

### Hot-Reload Example

```rust
use rusty_config::{Config, ConfigBuilder};
use tokio::time::sleep;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HotReloadConfig {
    message: String,
    counter: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a config with hot-reload
    let config = ConfigBuilder::new()
        .file("config.json")
        .hot_reload(true)
        .build::<HotReloadConfig>()
        .await?;

    // Start a watcher for changes
    let mut change_stream = config.watch_changes();
    
    loop {
        tokio::select! {
            // Wait for config changes
            Ok(new_config) = change_stream.recv() => {
                println!("🔄 Config updated!");
                println!("New message: {}", new_config.message);
            }
            
            // Periodic check
            _ = sleep(Duration::from_secs(5)) => {
                let current_config = config.get();
                println!("Current config: {:?}", current_config);
            }
        }
    }
}
```

### Validation

```rust
use rusty_config::{Config, ConfigBuilder, validator::{Validatable, TypeValidator}};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValidatedConfig {
    server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

// Implement validation
#[async_trait::async_trait]
impl Validatable for ValidatedConfig {
    async fn validate(&self) -> rusty_config::ConfigResult<()> {
        TypeValidator::not_empty(&self.server.host, "server.host")?;
        TypeValidator::port(self.server.port, "server.port")?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::new()
        .file("config.json")
        .validate_on_load(true)
        .build::<ValidatedConfig>()
        .await?;

    // Manual validation
    config.validate().await?;
    
    Ok(())
}
```

## 📁 Supported Formats

### JSON
```json
{
  "server": {
    "host": "localhost",
    "port": 8080
  },
  "database": {
    "url": "postgresql://localhost/myapp",
    "pool_size": 10
  }
}
```

### YAML
```yaml
server:
  host: localhost
  port: 8080
database:
  url: postgresql://localhost/myapp
  pool_size: 10
```

### TOML
```toml
[server]
host = "localhost"
port = 8080

[database]
url = "postgresql://localhost/myapp"
pool_size = 10
```

## 🛠️ API Reference

### ConfigBuilder

```rust
let config = ConfigBuilder::new()
    .file("config.yaml")           // Set file path
    .hot_reload(true)              // Enable hot-reload
    .validate_on_load(true)        // Validate on load
    .create_if_missing(true)       // Create file if missing
    .default_content(content)      // Set default content
    .build::<AppConfig>()          // Build config
    .await?;
```

### Config

```rust
let config = Config::from_file("config.json").await?;

// Get config
let data = config.get();

// Hot-reload stream
let mut changes = config.watch_changes();
while let Ok(new_config) = changes.recv().await {
    println!("Config changed: {:?}", new_config);
}

// Manual reload
config.reload().await?;

// Save
config.save().await?;

// Version and timestamp
println!("Version: {}", config.version());
println!("Last modified: {:?}", config.last_modified());
```

### Validation

```rust
use rusty_config::validator::{TypeValidator, CommonValidators};

// String validation
TypeValidator::not_empty(&value, "field_name")?;
TypeValidator::length(&value, 1, 100, "field_name")?;

// Numeric validation
TypeValidator::range(value, 1, 100, "field_name")?;
TypeValidator::port(port, "field_name")?;

// URL and email validation
TypeValidator::url(&url, "field_name")?;
TypeValidator::email(&email, "field_name")?;

// Predefined validators
CommonValidators::validate_server_config(&host, port)?;
CommonValidators::validate_database_config(&url, pool_size)?;
CommonValidators::validate_logging_config(&level)?;
```

## 🧪 Examples

The repository contains several examples:

```bash
# Basic usage
cargo run --example basic_usage

# Hot-reload demo
cargo run --example hot_reload

# Validation
cargo run --example validation
```

## 🔧 Features

### Available Features

```toml
[dependencies]
rusty-config = { version = "0.1.0", features = ["yaml", "json", "toml", "hot-reload", "validation", "logging"] }
```

- `yaml` - YAML support (default)
- `json` - JSON support (default)
- `toml` - TOML support (default)
- `hot-reload` - Hot-reload functionality
- `validation` - Validation functions
- `logging` - Logging integration

## 🤝 Contributing

Contributions are welcome! Please read the [Contributing Guidelines](CONTRIBUTING.md) first.

### Development

```bash
# Clone the repository
git clone https://github.com/yourusername/rusty-config.git
cd rusty-config

# Run tests
cargo test

# Generate documentation
cargo doc --open

# Linting
cargo clippy

# Formatting
cargo fmt
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgements

- [serde](https://github.com/serde-rs/serde) for serialization
- [tokio](https://github.com/tokio-rs/tokio) for the async runtime
- [notify](https://github.com/notify-rs/notify) for file watching
- [thiserror](https://github.com/dtolnay/thiserror) for error handling

## 📊 Project Status

- ✅ Basic functionality
- ✅ Hot-reload
- ✅ Validation
- ✅ Multi-format support
- ✅ Async API
- ✅ Builder pattern
- ✅ Comprehensive tests
- ✅ Documentation
- 🔄 CI/CD pipeline
- 🔄 Performance optimizations
- 🔄 Advanced validation rules

## 🚀 Roadmap

- [ ] Environment variable support
- [ ] Remote configuration (HTTP, etc.)
- [ ] Configuration encryption
- [ ] Configuration templates
- [ ] Performance benchmarks
- [ ] WebAssembly support
- [ ] Configuration migration tools

---

**Made with ❤️ in Rust** 