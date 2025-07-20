use rusty_config::ConfigBuilder;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HotReloadConfig {
    message: String,
    counter: u32,
    settings: Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    enabled: bool,
    interval: u64,
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 RustyConfig - Hot-Reload Beispiel");
    println!("==================================\n");

    // Erstelle eine Konfiguration mit Hot-Reload
    let config = ConfigBuilder::new()
        .file("hot_reload_config.json")
        .hot_reload(true)
        .create_if_missing(true)
        .default_content(
            r#"{
  "message": "Hallo von RustyConfig!",
  "counter": 0,
  "settings": {
    "enabled": true,
    "interval": 1000,
    "timeout": 5000
  }
}"#
            .to_string(),
        )
        .build::<HotReloadConfig>()
        .await?;

    println!("✅ Hot-Reload-Konfiguration gestartet!");
    println!("📁 Überwachte Datei: hot_reload_config.json");
    println!("🔄 Version: {version}", version = config.version());
    println!();

    // Starte einen Watcher für Konfigurationsänderungen
    let mut change_stream = config.watch_changes();

    // Zeige initiale Konfiguration
    let initial_config = config.get();
    println!("📊 Initiale Konfiguration:");
    println!("   Nachricht: {message}", message = initial_config.message);
    println!("   Zähler: {counter}", counter = initial_config.counter);
    println!("   Aktiviert: {enabled}", enabled = initial_config.settings.enabled);
    println!("   Intervall: {interval}ms", interval = initial_config.settings.interval);
    println!("   Timeout: {timeout}ms", timeout = initial_config.settings.timeout);
    println!();

    println!("👀 Warte auf Konfigurationsänderungen...");
    println!("   Bearbeite die Datei 'hot_reload_config.json' um Änderungen zu sehen!");
    println!("   Drücke Ctrl+C zum Beenden.");
    println!();

    // Hauptschleife für Hot-Reload
    let mut last_version = config.version();

    loop {
        tokio::select! {
            // Warte auf Konfigurationsänderungen
            Ok(new_config) = change_stream.recv() => {
                let current_version = config.version();
                if current_version > last_version {
                    println!("🔄 Konfiguration wurde aktualisiert! (v{last} -> v{current})",
                        last = last_version, current = current_version);
                    println!("📊 Neue Konfiguration:");
                    println!("   Nachricht: {message}", message = new_config.message);
                    println!("   Zähler: {counter}", counter = new_config.counter);
                    println!("   Aktiviert: {enabled}", enabled = new_config.settings.enabled);
                    println!("   Intervall: {interval}ms", interval = new_config.settings.interval);
                    println!("   Timeout: {timeout}ms", timeout = new_config.settings.timeout);
                    println!();

                    last_version = current_version;
                }
            }

            // Simuliere periodische Überprüfung
            _ = sleep(Duration::from_secs(5)) => {
                let current_config = config.get();
                println!("⏰ Status-Check (v{version}):", version = config.version());
                println!("   Nachricht: {message}", message = current_config.message);
                println!("   Zähler: {counter}", counter = current_config.counter);
                println!();
            }
        }
    }
}

// Hilfsfunktion zum Demonstrieren von Hot-Reload
#[allow(dead_code)]
async fn demonstrate_hot_reload() {
    println!("🎯 Hot-Reload-Demonstration:");
    println!("1. Ändere 'message' in der Konfigurationsdatei");
    println!("2. Erhöhe 'counter' um 1");
    println!("3. Ändere 'settings.enabled' auf false");
    println!("4. Beobachte die automatischen Updates!");
    println!();
}
