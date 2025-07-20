use rusty_config::{
    validator::{CommonValidators, TypeValidator, Validatable},
    ConfigBuilder,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValidatedConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    logging: LoggingConfig,
    api: ApiConfig,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiConfig {
    base_url: String,
    api_key: String,
    rate_limit: u32,
}

// Implementiere Validierung für die Konfiguration
#[async_trait::async_trait]
impl Validatable for ValidatedConfig {
    async fn validate(&self) -> rusty_config::ConfigResult<()> {
        // Validiere Server-Konfiguration
        TypeValidator::not_empty(&self.server.host, "server.host")?;
        TypeValidator::port(self.server.port, "server.port")?;
        TypeValidator::range(self.server.workers, 1, 100, "server.workers")?;

        // Validiere Datenbank-Konfiguration
        TypeValidator::not_empty(&self.database.url, "database.url")?;
        TypeValidator::range(self.database.pool_size, 1, 100, "database.pool_size")?;
        TypeValidator::range(self.database.timeout, 1, 300, "database.timeout")?;

        // Validiere Logging-Konfiguration
        CommonValidators::validate_logging_config(&self.logging.level)?;
        TypeValidator::not_empty(&self.logging.file, "logging.file")?;
        TypeValidator::range(self.logging.max_size, 1024, 1073741824, "logging.max_size")?; // 1KB bis 1GB

        // Validiere API-Konfiguration
        TypeValidator::url(&self.api.base_url, "api.base_url")?;
        TypeValidator::not_empty(&self.api.api_key, "api.api_key")?;
        TypeValidator::length(&self.api.api_key, 16, 256, "api.api_key")?;
        TypeValidator::range(self.api.rate_limit, 1, 10000, "api.rate_limit")?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("✅ RustyConfig - Validierungsbeispiel");
    println!("===================================\n");

    // Test 1: Gültige Konfiguration
    println!("🧪 Test 1: Gültige Konfiguration");
    let valid_config = ConfigBuilder::new()
        .file("valid_config.json")
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
  },
  "api": {
    "base_url": "https://api.example.com",
    "api_key": "sk-1234567890abcdef1234567890abcdef",
    "rate_limit": 1000
  }
}"#
            .to_string(),
        )
        .validate_on_load(true)
        .build::<ValidatedConfig>()
        .await?;

    println!("✅ Gültige Konfiguration erfolgreich validiert!");
    println!("📊 Konfiguration: {config:?}", config = valid_config.get());
    println!();

    // Test 2: Ungültige Konfiguration (Port 0)
    println!("🧪 Test 2: Ungültige Konfiguration (Port 0)");
    let invalid_config_result = ConfigBuilder::new()
        .file("invalid_config.json")
        .create_if_missing(true)
        .default_content(
            r#"{
  "server": {
    "host": "localhost",
    "port": 0,
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
  },
  "api": {
    "base_url": "https://api.example.com",
    "api_key": "sk-1234567890abcdef1234567890abcdef",
    "rate_limit": 1000
  }
}"#
            .to_string(),
        )
        .validate_on_load(true)
        .build::<ValidatedConfig>()
        .await;

    match invalid_config_result {
        Ok(_) => println!("❌ Erwarteter Fehler trat nicht auf!"),
        Err(e) => println!("✅ Erwarteter Validierungsfehler: {error}", error = e),
    }
    println!();

    // Test 3: Ungültige Konfiguration (leerer Host)
    println!("🧪 Test 3: Ungültige Konfiguration (leerer Host)");
    let invalid_config_result2 = ConfigBuilder::new()
        .file("invalid_config2.json")
        .create_if_missing(true)
        .default_content(
            r#"{
  "server": {
    "host": "",
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
  },
  "api": {
    "base_url": "https://api.example.com",
    "api_key": "sk-1234567890abcdef1234567890abcdef",
    "rate_limit": 1000
  }
}"#
            .to_string(),
        )
        .validate_on_load(true)
        .build::<ValidatedConfig>()
        .await;

    match invalid_config_result2 {
        Ok(_) => println!("❌ Erwarteter Fehler trat nicht auf!"),
        Err(e) => println!("✅ Erwarteter Validierungsfehler: {error}", error = e),
    }
    println!();

    // Test 4: Ungültige Konfiguration (ungültiges Log-Level)
    println!("🧪 Test 4: Ungültige Konfiguration (ungültiges Log-Level)");
    let invalid_config_result3 = ConfigBuilder::new()
        .file("invalid_config3.json")
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
    "level": "invalid_level",
    "file": "app.log",
    "max_size": 10485760
  },
  "api": {
    "base_url": "https://api.example.com",
    "api_key": "sk-1234567890abcdef1234567890abcdef",
    "rate_limit": 1000
  }
}"#
            .to_string(),
        )
        .validate_on_load(true)
        .build::<ValidatedConfig>()
        .await;

    match invalid_config_result3 {
        Ok(_) => println!("❌ Erwarteter Fehler trat nicht auf!"),
        Err(e) => println!("✅ Erwarteter Validierungsfehler: {error}", error = e),
    }
    println!();

    // Test 5: Manuelle Validierung
    println!("🧪 Test 5: Manuelle Validierung");
    let config = ConfigBuilder::new()
        .file("manual_validation_config.json")
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
  },
  "api": {
    "base_url": "https://api.example.com",
    "api_key": "sk-1234567890abcdef1234567890abcdef",
    "rate_limit": 1000
  }
}"#
            .to_string(),
        )
        .build::<ValidatedConfig>()
        .await?;

    match config.validate().await {
        Ok(_) => println!("✅ Manuelle Validierung erfolgreich!"),
        Err(e) => println!("❌ Validierungsfehler: {error}", error = e),
    }
    println!();

    println!("✨ Validierungsbeispiel erfolgreich abgeschlossen!");
    println!();
    println!("📋 Zusammenfassung der Validierungsregeln:");
    println!("   • Server-Host darf nicht leer sein");
    println!("   • Server-Port muss zwischen 1-65535 liegen");
    println!("   • Server-Workers müssen zwischen 1-100 liegen");
    println!("   • Datenbank-URL darf nicht leer sein");
    println!("   • Datenbank-Pool-Größe muss zwischen 1-100 liegen");
    println!("   • Log-Level muss gültig sein (trace, debug, info, warn, error)");
    println!("   • Log-Datei darf nicht leer sein");
    println!("   • Log-Max-Size muss zwischen 1KB-1GB liegen");
    println!("   • API-Base-URL muss gültige URL sein");
    println!("   • API-Key darf nicht leer sein und muss 16-256 Zeichen lang sein");
    println!("   • API-Rate-Limit muss zwischen 1-10000 liegen");

    Ok(())
}
