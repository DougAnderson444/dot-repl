//! Native storage
use crate::Error;
use directories::ProjectDirs;
use std::path::PathBuf;
use ui::PlatformStorage;

#[derive(Clone, Default)]
pub struct DesktopStorage {
    /// The [PathBuf] where the wallet data will be stored
    data_dir: PathBuf,
}

/// Storage implementation that saves data in a folder inside the git repo for version control.
#[derive(Clone, Default)]
pub struct GitStorage {
    repo_dir: PathBuf,
}

impl GitStorage {
    pub fn new(path_name: impl Into<PathBuf>) -> Result<Self, Error> {
        let repo_dir = path_name.into();
        std::fs::create_dir_all(&repo_dir).map_err(|err| {
            Error::StorageFailure(format!(
                "Failed to create repo_data directory {:?}: {:?}",
                repo_dir, err
            ))
        })?;
        Ok(Self { repo_dir })
    }
}

impl PlatformStorage for GitStorage {
    fn save(&self, key: &str, data: &[u8]) -> Result<(), String> {
        let path = self.repo_dir.join(key);
        std::fs::write(path, data).map_err(|err| format!("Failed to save data: {:?}", err))
    }

    fn load(&self, key: &str) -> Result<Vec<u8>, String> {
        let path = self.repo_dir.join(key);
        std::fs::read(path).map_err(|err| format!("Failed to load data: {:?}", err))
    }

    fn delete(&self, key: &str) -> Result<(), String> {
        let path = self.repo_dir.join(key);
        std::fs::remove_file(path).map_err(|err| format!("Failed to delete data: {:?}", err))
    }

    fn exists(&self, key: &str) -> bool {
        let path = self.repo_dir.join(key);
        path.exists()
    }
}

impl DesktopStorage {
    /// Creates a new instance of `DesktopStorage`, initializing the data directory.
    pub fn new() -> Result<Self, Error> {
        // If we run two apps at the same time, we need to use different directories
        // so that they don't interfere with each others' identities.
        let suffix = if std::env::var("DIOXUS_IDENTITY").is_ok() {
            "-second-app"
        } else {
            ""
        };
        let project_dirs = ProjectDirs::from("io", "peerpiper", &format!("dot-repl{}", suffix))
            .ok_or(Error::StorageFailure(
                "Failed to get project directories".to_string(),
            ))?;

        let data_dir = project_dirs.data_dir().to_path_buf();

        std::fs::create_dir_all(&data_dir).map_err(|err| {
            Error::StorageFailure(format!(
                "Failed to create data directory {:?}: {:?}",
                data_dir, err
            ))
        })?;

        Ok(Self { data_dir })
    }

    // /// Returns the directory where the wallet data is stored.
    // pub fn dir(&self) -> PathBuf {
    //     self.data_dir.clone()
    // }
}

impl PlatformStorage for DesktopStorage {
    fn save(&self, key: &str, data: &[u8]) -> Result<(), String> {
        let path = self.data_dir.join(key);
        std::fs::write(path, data).map_err(|err| format!("Failed to save data: {:?}", err))
    }

    fn load(&self, key: &str) -> Result<Vec<u8>, String> {
        let path = self.data_dir.join(key);
        std::fs::read(path).map_err(|err| format!("Failed to load data: {:?}", err))
    }

    fn delete(&self, key: &str) -> Result<(), String> {
        let path = self.data_dir.join(key);
        std::fs::remove_file(path).map_err(|err| format!("Failed to delete data: {:?}", err))
    }

    fn exists(&self, key: &str) -> bool {
        let path = self.data_dir.join(key);
        path.exists()
    }
}
