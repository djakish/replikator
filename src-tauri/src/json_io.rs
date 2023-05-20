use crate::types::{BackupEntry, Backups};
use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};
use std::{fs::File, path::Path};
use  xxhash_rust::xxh3::xxh3_64 as hash;

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
pub fn get_backups_to_update(json_path: &str) -> String {
    if Path::new(json_path).exists() {
        let file = File::open(json_path).unwrap();
        let backup_entries: Backups =
            serde_json::from_reader(file).expect("JSON was not formatted");

        let mut result: Vec<BackupEntry> = vec![];
        for backup in &backup_entries.backups {
            let last_backup: DateTime<Utc> = DateTime::from_utc(
                NaiveDateTime::parse_from_str(&backup.last_backup, "%Y-%m-%dT%H:%M:%SZ").unwrap(),
                Utc,
            );

            let next_backup =
                last_backup + Duration::days(backup.next_update.into());
            let now: DateTime<Utc> = Utc::now();
            let comparison_result = now.cmp(&next_backup);
            if comparison_result == std::cmp::Ordering::Greater {
                result.push(backup.clone());
            }
        }
        serde_json::to_string(&result).unwrap()
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
        last_backup: String::from("1971-02-10T13:00:00Z"),
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

pub fn change_date(json_path: &str, hash: String, new_date: String) {
    let file = File::open(json_path).unwrap();
    let mut json: Backups = serde_json::from_reader(file).expect("JSON was not formatted");

    let index = json
        .backups
        .iter()
        .position(|entry| *entry.hash == hash)
        .unwrap();

    json.backups[index].last_backup = new_date;

    std::fs::write(json_path, serde_json::to_string_pretty(&json).unwrap()).unwrap();
}
