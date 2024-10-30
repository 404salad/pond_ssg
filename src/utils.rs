use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::metadata;
use std::fs::ReadDir;
use std::io::Error;
use std::path::PathBuf;
use std::time::SystemTime;

pub fn content_directory_files() -> Vec<PathBuf> {
    let paths_result = match fs::read_dir("content") {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("couldnt read content directory: {e}");
            return vec![];
        }
    };
    let paths: Vec<PathBuf> = paths_result
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths
}
pub fn read_directory_content() -> Vec<String> {
    let mut article_names: Vec<String> = vec![];
    let paths_result = fs::read_dir("content");
    // iterating through paths
    match paths_result {
        Ok(paths) => {
            for path_result in paths {
                match path_result {
                    Ok(path) => {
                        if let Some(path_str) = path.path().to_str() {
                            article_names.push(
                                path_str
                                    .trim_start_matches("content/")
                                    .trim_end_matches(".md")
                                    .to_string(),
                            );
                        } else {
                            eprintln!("Error converting path to string");
                        }
                    }
                    Err(err) => {
                        eprintln!("Error reading directory entry: {}", err);
                        continue;
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
        }
    };
    article_names
}

pub fn files_changed(latest_change_times: &mut HashMap<PathBuf, SystemTime>) -> Vec<PathBuf> {
    let individual_files = content_directory_files();

    let new_change_times: HashMap<PathBuf, SystemTime> = individual_files
        .into_iter()
        .filter_map(|path| {
            if path.exists() {
                fs::metadata(&path)
                    .ok()
                    .and_then(|metadata| metadata.accessed().ok())
                    .map(|time| (path, time))
            } else {
                None
            }
        })
        .collect();

    let files_changed: Vec<PathBuf> = new_change_times
        .clone()
        .into_iter()
        .filter_map(|(path, time)| {
            if latest_change_times.get(&path) != Some(&time) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    latest_change_times.clear();
    latest_change_times.extend(new_change_times);
    files_changed
}
// TODO: GO THROUGH THE FILES AND STORE THEIR INITIAL VALUES, THEN CHECK FOR CHANGES

pub fn no_folder_level_changes(latest_change_time: &mut SystemTime) -> bool {
    let metadata = match fs::metadata("content") {
        Ok(meta) => {
            if !meta.is_dir() {
                eprintln!("Error: content is not a directory");
                return false;
            }
            meta
        }
        Err(e) => {
            eprintln!("Failed to read content metadata: {}", e);
            return false;
        }
    };

    // TODO: check compatibiliy before the first run to avoid matching for errors in unwrap
    let time = metadata.accessed().unwrap();
    if *latest_change_time == time {
        return true;
    } else {
        *latest_change_time = time;
        return false;
    }
}

pub fn has_content_dir() -> bool {
    if let Ok(current_directory) = env::current_dir() {
        current_directory.join("content").exists()
    } else {
        false
    }
}

pub fn delete_dir_contents(read_dir_res: Result<ReadDir, Error>) {
    println!("Removing previous content");
    if let Ok(dir) = read_dir_res {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(path).expect("Failed to remove a dir");
                } else {
                    fs::remove_file(path).expect("Failed to remove a file");
                }
            };
        }
    };
    println!("successfully removed previous content");
}
