//! This module defines the trait details for managing data.
use std::sync::Arc;

pub const STORAGE_KEY: &str = "kitchen_sink.dot";

static KITCHEN_SINK: &str = include_str!("../assets/dot/kitchen_sink.dot");

pub trait PlatformStorage: Send + Sync {
    fn save(&self, key: &str, data: &[u8]) -> Result<(), String>;
    fn load(&self, key: &str) -> Result<Vec<u8>, String>;
    fn delete(&self, key: &str) -> Result<(), String>;
    fn exists(&self, key: &str) -> bool;
}

// A storage provider context that wraps any storage implementation
#[derive(Clone)]
pub struct StorageProvider {
    inner: Arc<dyn PlatformStorage>,
}

impl StorageProvider {
    pub fn new<S: PlatformStorage + 'static>(storage: S) -> Self {
        if !storage.exists(STORAGE_KEY) {
            let _ = storage.save(STORAGE_KEY, KITCHEN_SINK.as_bytes());
        }

        Self {
            inner: Arc::new(storage),
        }
    }

    pub fn save(&self, key: &str, data: &[u8]) -> Result<(), String> {
        self.inner.save(key, data)
    }

    pub fn load(&self, key: &str) -> Result<Vec<u8>, String> {
        self.inner.load(key)
    }

    pub fn delete(&self, key: &str) -> Result<(), String> {
        self.inner.delete(key)
    }

    pub fn exists(&self, key: &str) -> bool {
        self.inner.exists(key)
    }
}
