#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

use std::fmt::format;
use std::io::{Read, Write};
// my imports
use jwalk::{Parallelism, WalkDir};
use std::cmp::Ordering;
use std::path::Path;
use std::path::PathBuf;

use std::ffi::OsStr;
use std::fs::{self, File};
use std::sync::{Arc, Mutex};
use std::{io, thread};
use tree_flat::prelude::*;
use zstd::stream::Encoder;

use flate2::write::GzEncoder;
use flate2::Compression;
use pathdiff::diff_paths;
use std::path::*;
use tauri::Window;

#[derive(Debug, thiserror::Error)]
enum FileTreeError {
    #[error("Failed to read files: {0}")]
    Io(#[from] std::io::Error),
}

impl serde::Serialize for FileTreeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

fn name_from_path(path: PathBuf) -> String {
    let os_str: &OsStr = path.file_name().unwrap();
    let string: String = os_str.to_string_lossy().to_string();
    format!("{}", string)
}

#[tauri::command]
async fn get_files_tree(path: &str) -> Result<(String, usize), FileTreeError> {
    let mut tree = Tree::new(path.to_string());
    let mut parent = tree.tree_root_mut().parent;
    for entry in WalkDir::new(path)
        .parallelism(Parallelism::RayonNewPool(4))
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
        }
    }

    let count = tree.as_data().len();

    Ok((format!("{}", tree),count))
}

// let file_path = PathBuf::from(output);
// let name = name_from_path(file_path);
// let new  = PathBuf::from(output).join(format!("{}.tar.zst", name));
// let tar_zst = File::create(new).unwrap();
// let enc = zstd::Encoder::new(tar_zst, 1).unwrap();
// let mut tar = tar::Builder::new(enc);
// tar.append_dir_all(input, output).unwrap();

#[tauri::command]
async fn compress_files(window: Window, input: &str, output: &str) -> Result<(), ()> {
    let input_dir = Path::new(input);

    for entry in WalkDir::new(input_dir)
        .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();
        if entry.path().is_file() {
            let dest_path = format!("{}/{}.zst", output, file_name);
            let input_file = File::open(entry.path()).unwrap();
            let output_file = File::create(dest_path).unwrap();

            zstd::stream::copy_encode(input_file, output_file, 3).unwrap();

            window.emit("compress://progress", Payload { message: format!("[compressed] {} ", file_name) }).unwrap();
        }
    }
    Ok(())
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

#[tauri::command]
async fn decompress_files(input: &str, output: &str) -> Result<(), ()> {
    let input_dir = Path::new(input);

    for entry in WalkDir::new(input_dir)
        .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();
        let new_string = &file_name[0..file_name.len() - 3];

        if entry.path().is_file() {
            let dest_path = format!("{}/{}", output, new_string);

            let input_file = File::open(entry.path()).unwrap();
            let output_file = File::create(dest_path).unwrap();

            zstd::stream::copy_decode(input_file, output_file).unwrap();

        }
    }
    Ok(())
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("open".to_string(), "Open");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_files_tree,
            compress_files,
            decompress_files
        ])
        .system_tray(tray)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
