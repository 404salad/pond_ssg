use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::ReadDir;
use std::io;
use std::io::{Result};
use std::path::{Path, PathBuf};
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
                            if path_str.ends_with(".md") {
                                article_names.push(
                                    path_str
                                        .trim_start_matches("content/")
                                        .trim_end_matches(".md")
                                        .to_string(),
                                );
                            }
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
    println!("{article_names:?}");
    article_names.sort_by_key(|k| time_of_creation(format!("content/{k}.md")));
    article_names.reverse();
    article_names
}

/// copy image files from the current directory to the `dist/articles` directory.
pub fn copy_image_files() -> io::Result<()> {
    let target_dir = Path::new("dist/articles");
    let image_extensions = ["png", "jpg", "jpeg", "gif", "bmp"];
    for entry in fs::read_dir("content")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
                if image_extensions.contains(&ext.to_lowercase().as_str()) {
                    let target_path = target_dir.join(path.file_name().unwrap());
                    fs::copy(&path, &target_path)?;
                    println!("Copied: {} -> {}", path.display(), target_path.display());
                }
            }
        }
    }
    Ok(())
}

pub fn time_of_creation(path: String) -> SystemTime {
    let time: SystemTime = SystemTime::now();
    match fs::metadata(path) {
        Ok(data) => data
            .created()
            .expect("chrnological sorting not supported on this platform"),
        Err(_) => time,
    }
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

pub fn delete_dir_contents(read_dir_res: Result<ReadDir>) {
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

pub fn create_code_formatting_files() -> std::io::Result<()>{
    let prismjs = include_str!("../styling_deps/prism.js");
    let prismcss = include_str!("../styling_deps/prism.css");
    fs::write("dist/prism.js", prismjs)?;
    fs::write("dist/prism.css", prismcss)?;
    Ok(())
}



// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_content_dir() {
        assert_eq!(has_content_dir(), true);
    }
}
