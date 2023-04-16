#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

// my imports
use jwalk::{Parallelism, WalkDir};
use std::path::Path;
use std::path::PathBuf;

use std::ffi::OsStr;
use std::fs::{self, File};
use tree_flat::prelude::*;

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

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn name_from_path(path: PathBuf) -> String {
    let os_str: &OsStr = path.file_name().unwrap();
    let string: String = os_str.to_string_lossy().to_string();
    string.to_string()
}

#[tauri::command]
async fn get_files_tree(path: &str) -> Result<(String, usize), FileTreeError> {
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


    // Making a directory with the same name as the input directory
    let together = format!(
        "{}\\{}",
        output,
        input_dir.file_name().unwrap().to_string_lossy()
    );
    let output_dir = Path::new(&together);
    
    fs::create_dir_all(output_dir).unwrap();

    for entry in WalkDir::new(input_dir)
        .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();
        let entry_path = entry.path();

        if entry_path.is_file() {
            let relative_path = entry_path.strip_prefix(input_dir).unwrap();

            let dest_path = match relative_path.parent() {
                Some(res) => {
                    fs::create_dir_all(output_dir.join(res)).unwrap();
                    format!(
                        "{}\\{}.zst",
                        output_dir.join(res).to_string_lossy(),
                        file_name
                    )
                }
                None => {
                    format!("{}\\{}.zst", output_dir.to_string_lossy(), file_name)
                }
            };

            let input_file = File::open(entry.path()).unwrap();
            let output_file = File::create(dest_path).unwrap();

            zstd::stream::copy_encode(input_file, output_file, 3).unwrap();

            window
                .emit(
                    "compress://progress",
                    Payload {
                        message: format!("[compressed] {} ", file_name),
                    },
                )
                .unwrap();
        }
    }
    Ok(())
}

#[tauri::command]
async fn decompress_files(window: Window, input: &str, output: &str) -> Result<(), ()> {
    let input_dir = Path::new(input);

    let together = format!(
        "{}\\{}",
        output,
        input_dir.file_name().unwrap().to_string_lossy()
    );

    let output_dir = Path::new(&together);

    // Create a parent directory
    fs::create_dir_all(output_dir).unwrap();

    for entry in WalkDir::new(input_dir)
        .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let old_name = entry.file_name().to_string_lossy();
        let file_name = &old_name[0..old_name.len() - 3];

        let entry_path = entry.path();

        if entry_path.is_file() {
            let relative_path = entry_path.strip_prefix(input_dir).unwrap();

            let dest_path = match relative_path.parent() {
                Some(res) => {
                    fs::create_dir_all(output_dir.join(res)).unwrap();
                    format!("{}\\{}", output_dir.join(res).to_string_lossy(), file_name)
                }
                None => {
                    format!("{}\\{}", output_dir.to_string_lossy(), file_name)
                }
            };

            let input_file = File::open(entry.path()).unwrap();
            let output_file = File::create(dest_path).unwrap();

            zstd::stream::copy_decode(input_file, output_file).unwrap();

            window
                .emit(
                    "compress://progress",
                    Payload {
                        message: format!("[decompressed] {} ", file_name),
                    },
                )
                .unwrap();
        }
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_files_tree,
            compress_files,
            decompress_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
