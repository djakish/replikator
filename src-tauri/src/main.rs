#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use jwalk::WalkDirGeneric;
// my imports
use jwalk::{Parallelism, WalkDir};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use tauri::Manager;

use seahash::hash;
use std::ffi::OsStr;
use std::fs::{self, read, File};
use tree_flat::prelude::*;

use tauri::Window;

#[derive(Debug, thiserror::Error)]
enum DirectoryReadError {
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
struct Payload {
    message: String,
}

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

// #[tauri::command]
// async fn create_dict(path: &str) -> Result<(), DirectoryReadError> {
//     let mut files : Vec<PathBuf> = vec![];
//     for entry in WalkDir::new(path)
//         .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
//         .into_iter()
//         .filter_map(|e| e.ok())
//     {
//         let path = entry.path();
//         if !path.is_dir() {
//             files.push(path)
//         }
//     }

//     let dict = zstd::dict::from_files(&files, 2048).unwrap();
//     let mut feed_dict_f = File::create(format!("{}\\dictionary", path) ).unwrap();
//     feed_dict_f.write_all(&dict).unwrap();

//     Ok(())
// }

#[tauri::command]
async fn compress_files(window: Window, input: &str, output: &str) -> Result<String, ()> {
    let start = std::time::Instant::now();
    let scene = std::sync::Arc::new(window);

    let input_dir = Path::new(input);

    // Extract the name of the input directory
    let input_dir_name = input_dir.file_name().unwrap().to_string_lossy();

    // Create the output directory with the same name as the input directory
    let output_dir = Path::new(output).join(&*input_dir_name);
    fs::create_dir_all(&output_dir).unwrap();

    let mut handles = vec![];
    // Iterate through each entry in the input directory
    for entry in WalkDir::new(input_dir)
        .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();
        let entry_path = entry.path();

        if entry_path.is_file() {
            let path_without_prefix = entry_path.strip_prefix(input_dir).unwrap();

            let dest_path = match path_without_prefix.parent() {
                Some(res) => {
                    // Basically if inside a folder just create a parent folder
                    // Technically there always will be a parent folder
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

            // Open the input and output files
            let input_file = File::open(entry.path()).unwrap();
            let output_file = File::create(dest_path.clone()).unwrap();

            // Compress the input file and write the output to the output file
            let clone_window = std::sync::Arc::clone(&scene);
            let handle = thread::spawn(move || {
                zstd::stream::copy_encode(input_file, output_file, 3).unwrap();
                // Emit progress event
                clone_window
                    .emit_all(
                        "compress://progress",
                        Payload {
                            message: format!("[compressed] {}", dest_path.clone()),
                        },
                    )
                    .unwrap();
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();

    Ok(format!("Backup took: {:?}", duration))
}


#[tauri::command]
async fn decompress_files(window: Window, input: &str, output: &str) -> Result<String, ()> {
    let start = std::time::Instant::now();

    let input_dir = Path::new(input);

    // Extract the name of the input directory
    let input_dir_name = input_dir.file_name().unwrap().to_string_lossy();

    // Create the output directory with the same name as the input directory
    let output_dir = Path::new(output).join(&*input_dir_name);
    fs::create_dir_all(&output_dir).unwrap();

    for entry in WalkDir::new(input_dir)
        .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let old_name = entry.file_name().to_string_lossy();
        let file_name = &old_name[0..old_name.len() - 3];

        let entry_path = entry.path();

        if entry_path.is_file() {
            let path_without_prefix = entry_path.strip_prefix(input_dir).unwrap();

            let dest_path = match path_without_prefix.parent() {
                Some(res) => {
                    // Basically if inside a folder just create a parent folder
                    // Technically there always will be a parent folder

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
    let duration = start.elapsed();

    Ok(format!("Restore took: {:?}", duration))
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
async fn hash_name(name: &str) -> Result<String, ()> {
    Ok(format!("{}", hash(name.to_string().as_bytes())))
}

#[tauri::command]
fn get_percentage_rounded(x: f32, y: f32) -> f32 {
    // Convert to rounded percentage string.
    let result = (x * 100.0) / y;
    result.round()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_files_tree,
            compress_files,
            decompress_files,
            hash_name,
            folder_compare,
            get_percentage_rounded
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
