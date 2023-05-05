use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    thread,
};

use jwalk::{Parallelism, WalkDir};
use tauri::{Manager, Window};

use crate::types::Payload;

fn add_extension(path: &mut std::path::PathBuf, extension: impl AsRef<std::path::Path>) {
    match path.extension() {
        Some(ext) => {
            let mut ext = ext.to_os_string();
            ext.push(".");
            ext.push(extension.as_ref());
            path.set_extension(ext)
        }
        None => path.set_extension(extension.as_ref()),
    };
}

#[tauri::command]
pub async fn compress_files(window: Window, input: &str, output: &str) -> Result<String, ()> {
    let start = std::time::Instant::now();
    let scene = std::sync::Arc::new(window);
    let input_dir = PathBuf::from(input);

    // Extract the name of the input directory
    let input_dir_name = input_dir.file_name().unwrap().to_string_lossy();

    // Create the output directory with the same name as the input directory
    let mut output_dir = PathBuf::from(output);
    output_dir.push(&*input_dir_name);
    fs::create_dir_all(&output_dir).unwrap();

    // handles for tasks
    let mut handles = vec![];
    // Iterate through each entry in the input directory
    for entry in WalkDir::new(&input_dir)
        .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();

        if entry_path.is_file() {
            let mut dest_path = output_dir.join(entry_path.strip_prefix(&input_dir).unwrap());

            // Append .zst extension
            add_extension(&mut dest_path, "zst");

            // Create parent directory if necessary
            fs::create_dir_all(dest_path.parent().unwrap()).unwrap();

            // Open the input and output files
            let input_file = File::open(entry_path).unwrap();
            let output_file = File::create(&dest_path).unwrap();

            // Compress the input file and write the output to the output file
            let clone_window = scene.clone();
            let handle = thread::spawn(move || {
                zstd::stream::copy_encode(input_file, output_file, 3).unwrap();
                // Emit progress event
                clone_window
                    .emit_all(
                        "compress://progress",
                        Payload {
                            message: format!("[compressed] {}", dest_path.display()),
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
pub async fn decompress_files(window: Window, input: &str, output: &str) -> Result<String, ()> {
    let start = std::time::Instant::now();
    let scene = std::sync::Arc::new(window);
    let input_dir = PathBuf::from(input);

    // Extract the name of the input directory
    let input_dir_name = input_dir.file_name().unwrap().to_string_lossy();

    // Create the output directory with the same name as the input directory
    let mut output_dir = PathBuf::from(output);
    output_dir.push(&*input_dir_name);
    fs::create_dir_all(&output_dir).unwrap();

    // handles for tasks
    let mut handles = vec![];
    for entry in WalkDir::new(&input_dir)
        .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();
        if entry_path.is_file() {
            let mut dest_path = output_dir.join(entry_path.strip_prefix(&input_dir).unwrap());

            // Strip .zst extension
            let file_stem = dest_path.file_stem().unwrap().to_str().unwrap();
            dest_path.set_file_name(file_stem.to_string());

            // Create parent directory if necessary
            fs::create_dir_all(dest_path.parent().unwrap()).unwrap();

            let input_file = File::open(entry.path()).unwrap();
            let output_file = File::create(dest_path.clone()).unwrap();

            // Compress the input file and write the output to the output file
            let clone_window = std::sync::Arc::clone(&scene);
            let handle = thread::spawn(move || {
                zstd::stream::copy_decode(input_file, output_file).unwrap();
                // Emit progress event
                clone_window
                    .emit_all(
                        "compress://progress",
                        Payload {
                            message: format!("[decompressed] {}", dest_path.display()),
                        },
                    )
                    .unwrap();
            });

            handles.push(handle);
        }
    }
    let duration = start.elapsed();

    Ok(format!("Restore took: {:?}", duration))
}

#[tauri::command]
pub async fn increment(
    window: Window,
    input: &str,
    output: &str,
    last_update: &str,
) -> Result<(), ()> {
    let scene = std::sync::Arc::new(window);
    let input_dir = Path::new(input);

    // Extract the name of the input directory
    let input_dir_name = input_dir.file_name().unwrap().to_string_lossy();

    // Create the output directory with the same name as the input directory
    let output_dir = Path::new(output).join(&*input_dir_name);
    fs::create_dir_all(&output_dir).unwrap();

    // handles for tasks
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
            let metadata = fs::metadata(entry_path.clone()).unwrap();
            // let tempmetadata.modified()

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

    Ok(())
}
