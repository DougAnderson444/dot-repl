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

    /// Save the hash of the last server-provided content for `key`.
    /// This is stored under a separate sentinel key so it never collides with
    /// user content.
    pub fn save_server_hash(&self, key: &str, hash: &str) {
        let sentinel = server_hash_key(key);
        // Ignore errors — this is best-effort bookkeeping.
        let _ = LocalStorage::set(&sentinel, hash);
    }

    /// Return the previously-stored server hash for `key`, if any.
    pub fn load_server_hash(&self, key: &str) -> Option<String> {
        let sentinel = server_hash_key(key);
        LocalStorage::get::<String>(&sentinel).ok()
    }
}

impl Default for WebStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// The `LocalStorage` key under which we store the server-side content hash
/// for a given user-facing key.  The `\0` separator makes it impossible for a
/// legitimate filename to collide with this key.
fn server_hash_key(key: &str) -> String {
    format!("{}\0__server_hash", key)
}

/// Compute a cheap, stable hash of a byte slice.
/// We use FNV-1a (64-bit) — it's tiny, deterministic, and has no external
/// dependencies beyond what we already pull in.
pub fn fnv1a_hex(data: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &byte in data {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", hash)
}
