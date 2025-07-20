use crate::ConfigResult;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
    time::SystemTime,
};
use tokio::sync::broadcast;
use uuid::Uuid;

/// Main configuration struct with hot-reload support
pub struct Config<T> {
    inner: Arc<RwLock<crate::watcher::ConfigData<T>>>,
    file_path: PathBuf,
    reload_tx: broadcast::Sender<T>,
    watcher_id: Uuid,
}

impl<T> Config<T>
where
    T: Clone + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Create a new config from a file
    pub async fn from_file<P: Into<PathBuf>>(path: P) -> ConfigResult<Self> {
        let path = path.into();
        let data = crate::loader::load_from_file(&path).await?;
        let (reload_tx, _reload_rx) = broadcast::channel(100);

        let config = Self {
            inner: Arc::new(RwLock::new(crate::watcher::ConfigData {
                data,
                last_modified: SystemTime::now(),
                version: 1,
            })),
            file_path: path,
            reload_tx,
            watcher_id: Uuid::new_v4(),
        };

        Ok(config)
    }

    /// Create a new config with hot-reload
    pub async fn from_file_with_watcher<P: Into<PathBuf>>(path: P) -> ConfigResult<Self> {
        let mut config = Self::from_file(path).await?;
        config.start_watcher().await?;
        Ok(config)
    }

    /// Start the file watcher for hot-reload
    async fn start_watcher(&mut self) -> ConfigResult<()> {
        crate::watcher::start_watcher(
            self.file_path.clone(),
            self.watcher_id,
            Arc::clone(&self.inner),
            self.reload_tx.clone(),
        )
        .await?;
        Ok(())
    }

    /// Reload the config from file
    pub async fn reload(&mut self) -> ConfigResult<()> {
        let new_data = crate::loader::load_from_file(&self.file_path).await?;

        {
            let mut inner = self.inner.write().unwrap();
            inner.data = new_data;
            inner.last_modified = SystemTime::now();
            inner.version += 1;
        }

        // Notify all listeners about the change
        let _ = self.reload_tx.send(self.get().clone());
        Ok(())
    }

    /// Get the current config
    pub fn get(&self) -> T {
        self.inner.read().unwrap().data.clone()
    }

    /// Get the current config as a reference
    pub fn get_ref(&self) -> std::sync::RwLockReadGuard<crate::watcher::ConfigData<T>> {
        self.inner.read().unwrap()
    }

    /// Get the config as a mutable reference
    pub fn get_mut(&mut self) -> std::sync::RwLockWriteGuard<crate::watcher::ConfigData<T>> {
        self.inner.write().unwrap()
    }

    /// Get a stream for config changes
    pub fn watch_changes(&self) -> broadcast::Receiver<T> {
        self.reload_tx.subscribe()
    }

    /// Get the version number of the current config
    pub fn version(&self) -> u64 {
        self.inner.read().unwrap().version
    }

    /// Get the last modified time
    pub fn last_modified(&self) -> SystemTime {
        self.inner.read().unwrap().last_modified
    }

    /// Save the config to file
    pub async fn save(&self) -> ConfigResult<()> {
        let data = self.get();
        crate::loader::save_to_file(&self.file_path, &data).await
    }

    /// Save the config to another file
    pub async fn save_to<P: Into<PathBuf>>(&self, path: P) -> ConfigResult<()> {
        let data = self.get();
        crate::loader::save_to_file(&path.into(), &data).await
    }

    /// Validate the current config
    pub async fn validate(&self) -> ConfigResult<()>
    where
        T: crate::validator::Validatable,
    {
        let data = self.get();
        crate::validator::validate(&data).await
    }
}

impl<T> Clone for Config<T>
where
    T: Clone + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            file_path: self.file_path.clone(),
            reload_tx: self.reload_tx.clone(),
            watcher_id: self.watcher_id,
        }
    }
}

impl<T> std::fmt::Debug for Config<T>
where
    T: std::fmt::Debug + Clone + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("data", &self.get())
            .field("file_path", &self.file_path)
            .field("version", &self.version())
            .field("watcher_id", &self.watcher_id)
            .finish()
    }
}
