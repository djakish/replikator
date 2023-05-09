use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BackupEntry {
    pub title: String,
    pub input: String,
    pub output: String,
    #[serde(rename(serialize = "lastBackup", deserialize = "lastBackup"))]
    pub last_backup: String,
    #[serde(rename(serialize = "nextUpdate", deserialize = "nextUpdate"))]
    pub next_update: u32,
    #[serde(rename(serialize = "deleteButton", deserialize = "deleteButton"))]
    pub delete_button: String,
    #[serde(rename(serialize = "backupButton", deserialize = "backupButton"))]
    pub backup_button: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct Backups {
    pub backups: Vec<BackupEntry>,
}

#[derive(Debug, thiserror::Error)]
pub enum DirectoryReadError {
    #[error("Failed to read files: {0}")]
    Io(#[from] std::io::Error),
}

impl serde::Serialize for DirectoryReadError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub message: String,
}
