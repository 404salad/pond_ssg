pub mod config;
pub mod consolidate_into_homepage;
pub mod parse_one_article;
pub mod utils;

use rayon::prelude::*;
use std::fs;

use console::Term;

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

    config::initial_config();

    let user_config = config::read_config().unwrap();
    //println!("{}", user_config);

    // Remove previous content (clean)
    let article_dir = fs::read_dir("dist/articles");
    utils::delete_dir_contents(article_dir);

    let article_names = consolidate_into_homepage::read_directory_content();
    // println!("{:?}", article_names);

    // rebuilding all the articles in content directory (parallely)

    article_names.par_iter().for_each(|article_name| {
        let user_config_for_threads = user_config.clone();
        match parse_one_article::markdown_to_styled_html(&article_name, &user_config_for_threads) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("unsuccesful parse {}", e)
            }
        }
    });
    println!("generated all blogs");

    match consolidate_into_homepage::create_homepage(&user_config) {
        Ok(_) => {
            println!("added all blogs to homepage, view in dist/index.html")
        }
        Err(e) => {
            eprintln!("unsuccesful in creating homepage {}", e)
        }
    };
}
