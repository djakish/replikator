use crate::types::{BackupEntry, Backups};
use chrono::{DateTime, Local};
use seahash::hash;
use std::{fs::File, path::Path};

#[tauri::command]
pub fn get_table(json_path: &str) -> String {
    if Path::new(json_path).exists() {
        let file = File::open(json_path).unwrap();
        let backup_entries: Backups =
            serde_json::from_reader(file).expect("JSON was not formatted");
        serde_json::to_string(&backup_entries).unwrap()
    } else {
        File::create(json_path).unwrap();
        let backup_entries: Backups = Backups { backups: vec![] };
        std::fs::write(
            json_path,
            serde_json::to_string_pretty(&backup_entries).unwrap(),
        )
        .unwrap();
        serde_json::to_string(&backup_entries).unwrap()
    }
}

#[tauri::command]
pub fn add_entry(json_path: &str, title: String, input: String, output: String, next_update: u32) {
    let local: DateTime<Local> = Local::now();
    let hash = hash(local.to_string().as_bytes());
    let file = File::open(json_path).unwrap();
    let mut json: Backups = serde_json::from_reader(file).expect("JSON was not formatted");
    let new_entry = BackupEntry {
        title,
        input,
        output,
        last_backup: String::new(),
        next_update,
        delete_button: String::new(),
        backup_button: String::new(),
        hash: hash.to_string(),
    };
    json.backups.push(new_entry);
    std::fs::write(json_path, serde_json::to_string_pretty(&json).unwrap()).unwrap();
}

#[tauri::command]
pub fn delete_entry(json_path: &str, hash: String) {
    let file = File::open(json_path).unwrap();
    let mut json: Backups = serde_json::from_reader(file).expect("JSON was not formatted");

    let index = json
        .backups
        .iter()
        .position(|entry| *entry.hash == hash)
        .unwrap();
    json.backups.remove(index);

    std::fs::write(json_path, serde_json::to_string_pretty(&json).unwrap()).unwrap();
}
