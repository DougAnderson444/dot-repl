use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use dot_repl_ui::PlatformStorage;
use gloo_storage::{LocalStorage, Storage};

#[derive(Clone)]
pub struct WebStorage;

impl PlatformStorage for WebStorage {
    fn save(&self, key: &str, data: &[u8]) -> Result<(), String> {
        // Convert binary data to Base64 string for storage
        let encoded = STANDARD_NO_PAD.encode(data);
        LocalStorage::set(key, encoded).map_err(|err| format!("Failed to save data: {:?}", err))
    }

    fn load(&self, key: &str) -> Result<Vec<u8>, String> {
        // Retrieve Base64 string and convert back to binary
        let encoded: String =
            LocalStorage::get(key).map_err(|err| format!("Failed to load data: {:?}", err))?;

        STANDARD_NO_PAD
            .decode(&encoded)
            .map_err(|err| format!("Failed to decode data: {:?}", err))
    }

    fn delete(&self, key: &str) -> Result<(), String> {
        // Remove the key from local storage
        LocalStorage::delete(key);
        Ok(())
    }

    fn exists(&self, key: &str) -> bool {
        // Check if key exists in local storage
        LocalStorage::get::<String>(key).is_ok()
    }
}

// You might also want to add a constructor
impl WebStorage {
    pub fn new() -> Self {
        WebStorage
    }
}
