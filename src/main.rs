mod config;
pub mod consolidate_into_homepage;
pub mod file_utils;
pub mod parse_one_article;

use config::UserConfig;
use console::Term;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

fn main() {
    let term = Term::stdout();
    let _ = term.clear_screen();
    let _ = term.write_line(
        "
██████   ██████  ███    ██ ██████      ███████ ███████  ██████  
██   ██ ██    ██ ████   ██ ██   ██     ██      ██      ██       
██████  ██    ██ ██ ██  ██ ██   ██     ███████ ███████ ██   ███ 
██      ██    ██ ██  ██ ██ ██   ██          ██      ██ ██    ██ 
██       ██████  ██   ████ ██████      ███████ ███████  ██████  
",
    );
    let _ = term.write_line("       A simple cli tool to convert markdown to blog");

    if file_utils::has_content_dir() {
        println!("content/ found");
    } else {
        println!("kindly make a content/ directory (check if you are running from root dir of the project)");
        return;
    }

    config::initial_config();

    let user_config = config::read_config().unwrap();
    println!("User Config: \n{user_config}");

    if user_config.code_formatting {
        if let Err(e) = file_utils::create_code_formatting_files() {
            eprintln!("Error creating support files for code formatting, consider setting code_formatting off from config.toml {e}"); // TODO only log error dont show?
        }
    }

    let cl_config = config::read_cl_args();

    if cl_config.help {
        println!("This is a cli tool for converting markdown to blog");
        println!("Put your markdown blogs in /content");
        println!("configure name and author in config.toml");
        println!("use the --watch flag for auto-running on detecting changes");
        println!("use the --help flag for this menu");
        println!("use the --silent flag to suppress all stdout");
        return;
    }

    if cl_config.silent {
        todo!("use a logger to impl");
    }

    if cl_config.watcher {
        // setting initial time for watching changes
        let mut folder_level_change_time = SystemTime::now();
        let individual_files = file_utils::content_directory_files();
        let mut file_level_change_times: HashMap<PathBuf, SystemTime> = individual_files
            .into_iter()
            .filter_map(|path| {
                if path.exists() && path.extension().and_then(|ext| ext.to_str()) == Some("md") {
                    fs::metadata(&path)
                        .ok()
                        .and_then(|metadata| metadata.accessed().ok())
                        .map(|time| (path, time))
                } else {
                    None
                }
            })
            .collect();

        // loop this based on file changes
        loop {
            if file_utils::no_folder_level_changes(&mut folder_level_change_time) {
                let files_changed = file_utils::files_changed(&mut file_level_change_times);
                if files_changed.is_empty() {
                    continue;
                }
                render_some(&user_config, &files_changed);
            } else {
                render_all(&user_config);
            }
        }
    }
    else {
        render_all(&user_config);

    }
}

fn render_some(user_config: &UserConfig, files_changed: &Vec<PathBuf>) {
    // remove previous content (clean)
    // no need of deleting old since we will overwrite them
    // this is different since we are doing it for each file atomically

    let _ = file_utils::copy_image_files();
    let article_names: Vec<String> = files_changed
        .iter()
        .filter_map(|path| path.file_name())
        .filter_map(|osstr| osstr.to_str())
        .filter(|path| path.ends_with(".md"))
        .map(|path| path.trim_end_matches(".md"))
        .map(String::from)
        .collect();

    println!("RENDERING SOME");
    println!("generated {article_names:?} blogs");
    println!("---");

    article_names.par_iter().for_each(|article_name| {
        let user_config_for_threads = user_config.clone();
        match parse_one_article::markdown_to_styled_html(&article_name, &user_config_for_threads) {
            Ok(_) => {
                println!("successful parse for {article_name}")
            }
            Err(e) => {
                eprintln!(" unsuccessful parse for {article_name}: {e}")
            }
        }
    });

    match consolidate_into_homepage::create_homepage(&user_config) {
        Ok(_) => {
            println!("partial refresh successfully!, view in dist/index.html")
        }
        Err(e) => {
            eprintln!("unsuccessful in creating homepage {e}")
        }
    };
}

fn render_all(user_config: &UserConfig) {
    println!("RENDERING ALL");
    println!("---");

    let _ = file_utils::copy_image_files();
    // remove previous content (clean)
    let article_dir = fs::read_dir("dist/articles");
    file_utils::delete_dir_contents(article_dir);

    let article_names = file_utils::read_directory_content();
    //println!("{:?}", article_names);

    // rebuilding all the articles in content directory (parallel)

    article_names.par_iter().for_each(|article_name| {
        let user_config_for_threads = user_config.clone();
        match parse_one_article::markdown_to_styled_html(&article_name, &user_config_for_threads) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("unsuccessful parse for {article_name} {}", e)
            }
        }
    });
    println!("generated all blogs");

    match consolidate_into_homepage::create_homepage(&user_config) {
        Ok(_) => {
            println!("added all blogs to homepage, view in dist/index.html")
        }
        Err(e) => {
            eprintln!("unsuccessful in creating homepage {}", e)
        }
    };
}
