use crate::{ConfigError, ConfigResult};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::sync::broadcast;
use uuid::Uuid;

#[cfg(feature = "hot-reload")]
use notify::event::ModifyKind;

/// Internal config data for the watcher
pub struct ConfigData<T> {
    pub data: T,
    pub last_modified: std::time::SystemTime,
    pub version: u64,
}

/// Start a file watcher for hot-reload
#[cfg(feature = "hot-reload")]
pub async fn start_watcher<T>(
    file_path: PathBuf,
    _watcher_id: Uuid,
    config_data: Arc<RwLock<ConfigData<T>>>,
    reload_tx: broadcast::Sender<T>,
) -> ConfigResult<()>
where
    T: Clone + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    use notify::{RecommendedWatcher, RecursiveMode, Watcher};

    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    let file_path_clone = file_path.clone();

    // Start the watcher in a separate thread
    std::thread::spawn(move || {
        let (notify_tx, notify_rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(notify_tx, notify::Config::default())
            .expect("Watcher could not be created");

        // Watch the config file
        if let Err(e) = watcher.watch(&file_path_clone, RecursiveMode::NonRecursive) {
            eprintln!("Error watching file: {:?}", e);
            return;
        }

        // Wait for notifications
        for res in notify_rx {
            match res {
                Ok(event) => {
                    if let Err(e) = tx.blocking_send(event) {
                        eprintln!("Error sending notification: {:?}", e);
                    }
                }
                Err(e) => eprintln!("Watcher error: {:?}", e),
            }
        }
    });

    // Process notifications asynchronously
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            if should_reload(&event, &file_path) {
                if let Err(e) = handle_file_change(&file_path, &config_data, &reload_tx).await {
                    eprintln!("Error reloading config: {:?}", e);
                }
            }
        }
    });

    Ok(())
}

#[cfg(not(feature = "hot-reload"))]
pub async fn start_watcher<T>(
    _file_path: PathBuf,
    _watcher_id: Uuid,
    _config_data: Arc<RwLock<ConfigData<T>>>,
    _reload_tx: broadcast::Sender<T>,
) -> ConfigResult<()>
where
    T: Clone + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    Err(ConfigError::HotReload(
        "Hot-reload feature is not enabled".to_string(),
    ))
}

/// Check if a file change should trigger a reload
#[cfg(feature = "hot-reload")]
fn should_reload(event: &notify::Event, file_path: &PathBuf) -> bool {
    // Check if the changed file is our config file
    if !event.paths.iter().any(|path| path == file_path) {
        return false;
    }

    // Check the event type
    match event.kind {
        notify::EventKind::Modify(ModifyKind::Data(_)) => true,
        notify::EventKind::Modify(ModifyKind::Metadata(_)) => true,
        notify::EventKind::Create(_) => true,
        _ => false,
    }
}

/// Handle a file change
#[allow(dead_code)]
async fn handle_file_change<T>(
    file_path: &PathBuf,
    config_data: &Arc<RwLock<ConfigData<T>>>,
    reload_tx: &broadcast::Sender<T>,
) -> ConfigResult<()>
where
    T: Clone + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    // Wait a bit to ensure the file is fully written
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Load the new config
    let new_data: T = crate::loader::load_from_file(file_path).await?;

    // Update the config data
    {
        let mut data = config_data.write().unwrap();
        data.data = new_data.clone();
        data.last_modified = std::time::SystemTime::now();
        data.version += 1;
    }

    // Notify all listeners
    let _ = reload_tx.send(new_data);

    Ok(())
}

/// Watcher manager for multiple files
pub struct ConfigWatcherManager {
    #[cfg(feature = "hot-reload")]
    watchers: std::collections::HashMap<Uuid, tokio::task::JoinHandle<()>>,
}

impl ConfigWatcherManager {
    /// Create a new watcher manager
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "hot-reload")]
            watchers: std::collections::HashMap::new(),
        }
    }

    /// Add a new watcher
    pub async fn add_watcher<T>(
        &mut self,
        _file_path: PathBuf,
        _watcher_id: Uuid,
        _config_data: Arc<RwLock<ConfigData<T>>>,
        _reload_tx: broadcast::Sender<T>,
    ) -> ConfigResult<()>
    where
        T: Clone + DeserializeOwned + Serialize + Send + Sync + 'static,
    {
        #[cfg(feature = "hot-reload")]
        {
            let handle = tokio::spawn(async move {
                if let Err(e) = start_watcher(_file_path, _watcher_id, _config_data, _reload_tx).await {
                    eprintln!("Error starting watcher: {:?}", e);
                }
            });

            self.watchers.insert(_watcher_id, handle);
            Ok(())
        }

        #[cfg(not(feature = "hot-reload"))]
        {
            Err(ConfigError::HotReload(
                "Hot-reload feature is not enabled".to_string(),
            ))
        }
    }

    /// Remove a watcher
    pub async fn remove_watcher(&mut self, _watcher_id: Uuid) {
        #[cfg(feature = "hot-reload")]
        {
            if let Some(handle) = self.watchers.remove(&_watcher_id) {
                handle.abort();
            }
        }
    }

    /// Stop all watchers
    pub async fn stop_all(&mut self) {
        #[cfg(feature = "hot-reload")]
        {
            for (_, handle) in self.watchers.drain() {
                handle.abort();
            }
        }
    }

    /// Get the number of active watchers
    pub fn active_watchers(&self) -> usize {
        #[cfg(feature = "hot-reload")]
        {
            self.watchers.len()
        }
        #[cfg(not(feature = "hot-reload"))]
        {
            0
        }
    }
}

impl Default for ConfigWatcherManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ConfigWatcherManager {
    fn drop(&mut self) {
        // Stop all watchers on drop
        #[cfg(feature = "hot-reload")]
        {
            for (_, handle) in self.watchers.drain() {
                handle.abort();
            }
        }
    }
}

/// Utility functions for watcher
pub mod utils {
    use super::*;

    /// Check if a file exists and is readable
    pub async fn is_file_readable(path: &PathBuf) -> bool {
        tokio::fs::metadata(path).await.is_ok()
    }

    /// Wait for a file to become available
    pub async fn wait_for_file(path: &PathBuf, timeout: Duration) -> ConfigResult<()> {
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            if is_file_readable(path).await {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        Err(ConfigError::Timeout)
    }

    /// Get the last modified time of a file
    pub async fn get_file_modified_time(path: &PathBuf) -> ConfigResult<std::time::SystemTime> {
        let metadata = tokio::fs::metadata(path).await?;
        metadata
            .modified()
            .map_err(|e| ConfigError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use tempfile::NamedTempFile;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestConfig {
        value: String,
    }

    #[tokio::test]
    async fn test_watcher_manager() {
        let mut manager = ConfigWatcherManager::new();
        assert_eq!(manager.active_watchers(), 0);

        // Test with a temporary file
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();
        let watcher_id = Uuid::new_v4();
        let config_data = Arc::new(RwLock::new(ConfigData {
            data: TestConfig {
                value: "initial".to_string(),
            },
            last_modified: std::time::SystemTime::now(),
            version: 1,
        }));
        let (reload_tx, _) = broadcast::channel(100);

        // Add watcher (only if hot-reload feature is enabled)
        let result = manager
            .add_watcher(path, watcher_id, config_data, reload_tx)
            .await;

        #[cfg(feature = "hot-reload")]
        {
            assert!(result.is_ok());
            assert_eq!(manager.active_watchers(), 1);

            // Remove watcher
            manager.remove_watcher(watcher_id).await;
            assert_eq!(manager.active_watchers(), 0);
        }

        #[cfg(not(feature = "hot-reload"))]
        {
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn test_file_utils() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        // Test is_file_readable
        assert!(utils::is_file_readable(&path).await);

        // Test get_file_modified_time
        let modified_time = utils::get_file_modified_time(&path).await.unwrap();
        assert!(modified_time > std::time::SystemTime::UNIX_EPOCH);
    }
}
