use rusty_config::{Config, ConfigBuilder};
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DatabaseConfig {
    url: String,
    pool_size: u32,
    timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoggingConfig {
    level: String,
    file: String,
    max_size: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 RustyConfig - Basic Example");
    println!("==============================\n");

    // Create a config using the builder
    let config = ConfigBuilder::new()
        .file("config.json")
        .create_if_missing(true)
        .default_content(
            r#"{
  "server": {
    "host": "localhost",
    "port": 8080,
    "workers": 4
  },
  "database": {
    "url": "postgresql://localhost/myapp",
    "pool_size": 10,
    "timeout": 30
  },
  "logging": {
    "level": "info",
    "file": "app.log",
    "max_size": 10485760
  }
}"#
            .to_string(),
        )
        .build::<AppConfig>()
        .await?;

    println!("✅ Config loaded successfully!");
    println!("📁 File: config.json");
    println!("🔄 Version: {}", config.version());
    println!();

    // Show config data
    let app_config = config.get();
    println!("📊 Config data:");
    println!(
        "   Server: {}:{} ({} workers)",
        app_config.server.host, app_config.server.port, app_config.server.workers
    );
    println!(
        "   Database: {} (Pool: {}, Timeout: {}s)",
        app_config.database.url, app_config.database.pool_size, app_config.database.timeout
    );
    println!(
        "   Logging: {} -> {} (Max: {} bytes)",
        app_config.logging.level, app_config.logging.file, app_config.logging.max_size
    );
    println!();

    // Change the config
    let mut updated_config = app_config.clone();
    updated_config.server.port = 9090;
    updated_config.logging.level = "debug".to_string();

    // Save the changes directly
    fs::write(
        "config_updated.json",
        serde_json::to_string_pretty(&updated_config)?,
    )
    .await?;

    println!("💾 Config saved to 'config_updated.json'!");
    println!("   New port: {}", updated_config.server.port);
    println!("   New log level: {}", updated_config.logging.level);
    println!();

    // Show supported formats
    println!("📝 Supported formats:");
    println!("   JSON: config.json");
    println!("   YAML: config.yaml");
    println!("   TOML: config.toml");
    println!();

    println!("✨ Example finished successfully!");
    Ok(())
}
