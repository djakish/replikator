#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// my imports
use jwalk::{Parallelism, WalkDir};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use seahash::hash;
use std::ffi::OsStr;
use std::fs::read;
use tree_flat::prelude::*;

mod compression;
mod json_io;
mod types;
use compression::*;
use json_io::*;
use types::*;

fn name_from_path(path: PathBuf) -> String {
    let os_str: &OsStr = path.file_name().unwrap();
    let string: String = os_str.to_string_lossy().to_string();
    string
}

#[tauri::command]
async fn get_files_tree(path: &str) -> Result<(String, usize), DirectoryReadError> {
    let mut tree = Tree::new(path.to_string());
    let mut parent = tree.tree_root_mut().parent;
    let mut count: usize = 0;
    for entry in WalkDir::new(path)
        .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.depth() == 0 {
            continue;
        }
        let node_id = tree.push_with_level(
            name_from_path(entry.path().to_path_buf()),
            entry.depth(),
            parent,
        );
        if entry.path().is_dir() {
            parent = node_id;
        } else {
            count += 1;
        }
    }

    Ok((format!("{}", tree), count))
}

#[tauri::command]
async fn folder_compare(input: &str, output: &str) -> Result<(String, usize), DirectoryReadError> {
    //Firstly we take the destination folder and put it in hash map
    let mut output_map = HashMap::new();

    let input_dir = Path::new(input);
    let output_dir = Path::new(output);

    for entry in WalkDir::new(output)
        .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path: PathBuf = entry.path().clone();
        let path_without_prefix: std::borrow::Cow<Path> = std::borrow::Cow::Owned(
            path.strip_prefix(output_dir)
                .expect("Failed to strip prefix")
                .to_path_buf(),
        );
        if !path.clone().is_dir() {
            let content = read(&path).unwrap();
            let hash = hash(&content);
            output_map.insert(path_without_prefix, hash);
        }
    }

    let mut tree = Tree::new(input.to_string());
    let mut parent = tree.tree_root_mut().parent;
    let mut count: usize = 0;
    for entry in WalkDir::new(input)
        .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.depth() == 0 {
            continue;
        }
        let file_name = name_from_path(entry.path().to_path_buf());
        let prefix: String;
        let path = entry.path().clone();
        let path_without_prefix: &Path = path.strip_prefix(input_dir).unwrap();
        if output_map.contains_key(<&std::path::Path>::clone(&path_without_prefix)) {
            let content = read(&path).unwrap();
            let hash = hash(&content);

            if output_map.get(path_without_prefix).unwrap() == &hash {
                prefix = "".to_owned();
            } else {
                prefix = "[CHANGED] ".to_owned();
            }
        } else {
            prefix = "[NEW] ".to_owned();
        }

        let together = format!("{}{}", prefix, file_name);
        let node_id = tree.push_with_level(together, entry.depth(), parent);
        if path.is_dir() {
            parent = node_id;
        } else {
            count += 1;
        }
    }

    Ok((format!("{}", tree), count))
}

#[tauri::command]
fn get_percentage_rounded(x: f32, y: f32) -> f32 {
    let result = (x * 100.0) / y;
    result.round()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_files_tree,
            compress_files,
            decompress_files,
            folder_compare,
            get_percentage_rounded,
            get_table,
            add_entry,
            delete_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
