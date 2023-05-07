use std::{
    fs::{self, File},
    path::PathBuf,
    thread,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use jwalk::{Parallelism, WalkDir};
use tauri::api::notification::Notification;
use tauri::{Manager, Window};

use crate::{json_io::change_date, types::Payload};

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
    json_path: &str,
    hash: String,
    input: &str,
    output: &str,
    last_backup: &str,
) -> Result<(), ()> {
    let date_time: DateTime<Utc> = DateTime::from_utc(
        NaiveDateTime::parse_from_str(last_backup, "%Y-%m-%dT%H:%M:%SZ").unwrap(),
        Utc,
    );

    let input_dir = PathBuf::from(input);

    // Extract the name of the input directory
    let input_dir_name = input_dir.file_name().unwrap().to_string_lossy();

    // Create the output directory with the same name as the input directory
    let mut output_dir = PathBuf::from(output);
    output_dir.push(&*input_dir_name);
    fs::create_dir_all(&output_dir).unwrap();

    // handles for tasks
    // Iterate through each entry in the input directory
    for entry in WalkDir::new(&input_dir)
        .parallelism(Parallelism::RayonNewPool(num_cpus::get()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();

        // Compare the file
        // Check if file exists in the output directory
        // If file exists we compare if its the same size
        // And compare if it has been modified before
        // If it doesnt exist we backup
        // If has been modified we backup
        // If file has been changed we backup

        if entry_path.is_file() {
            let mut should_backup = false;

            let mut dest_path = output_dir.join(entry_path.strip_prefix(&input_dir).unwrap());

            // The last modified time is before the parsed date
            // Append .zst extension
            add_extension(&mut dest_path, "zst");

            // Create parent directory if necessary
            fs::create_dir_all(dest_path.parent().unwrap()).unwrap();

            // Checking if file exists in the destination
            let file_exist = std::path::Path::new(&dest_path).exists();

            // If it does exist we check the size and date
            if file_exist {
                // Checking if file has
                let meta = fs::metadata(&dest_path).unwrap();

                let modified: DateTime<Utc> = meta.modified().unwrap().into();

                let comparison_result = modified.cmp(&date_time);

                if comparison_result == std::cmp::Ordering::Greater {
                    should_backup = true;
                } else {
                    let exact_size =
                        zstd_safe::find_decompressed_size(&fs::read(&dest_path).unwrap()).unwrap();

                    if let Some(res) = exact_size {
                        let existing_size = fs::metadata(&entry_path).unwrap().len();
                        if existing_size != res {
                            should_backup = true;
                        }
                    }
                }
            } else {
                should_backup = true;
            }
            // Open the input and output files
            if should_backup {
                let mut input_file = File::open(&entry_path).unwrap();

                let mut encoder = {
                    let target = fs::File::create(&dest_path).unwrap();
                    zstd::Encoder::new(target, 3).unwrap()
                };

                encoder
                    .multithread(num_cpus::get_physical() as u32)
                    .unwrap();

                std::io::copy(&mut input_file, &mut encoder).unwrap();
                encoder.finish().unwrap();
            }
        }
    }

    // update the lastBackup element and make a windows notification
    change_date(
        json_path,
        hash,
        Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    );

    Notification::new(String::from("com.djakish.dev"))
        .title("Repliktor")
        .body("The backup have finished.")
        .show()
        .unwrap();

    Ok(())
}
