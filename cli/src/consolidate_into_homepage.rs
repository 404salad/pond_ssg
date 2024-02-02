use std::fs;

pub fn read_directory_content() -> Vec<String>{
    let mut article_names:Vec<String> = vec![];
    let paths_result = fs::read_dir("content");
    // iterating through paths
    match paths_result {
        Ok(paths) => {
            for path_result in paths {
                match path_result {
                    Ok(path) => {
                        if let Some(path_str) = path.path().to_str() {
                            article_names.push(path_str
                                               .trim_start_matches("content/")
                                               .trim_end_matches(".md")
                                               .to_string());
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
