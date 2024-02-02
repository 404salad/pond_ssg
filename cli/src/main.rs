pub mod parse_one_article;
pub mod consolidate_into_homepage;

use std::thread;
use std::time::Duration;

use console::Term;

fn main() {
     let term = Term::stdout();
    term.clear_screen();
    term.write_line("
    ██████   ██████  ███    ██ ██████      ███████ ███████  ██████  
    ██   ██ ██    ██ ████   ██ ██   ██     ██      ██      ██       
    ██████  ██    ██ ██ ██  ██ ██   ██     ███████ ███████ ██   ███ 
    ██      ██    ██ ██  ██ ██ ██   ██          ██      ██ ██    ██ 
    ██       ██████  ██   ████ ██████      ███████ ███████  ██████  
                    ");
    thread::sleep(Duration::from_millis(200));
    term.write_line("       A simple cli tool to convert markdown to blog");

    let article_names = consolidate_into_homepage::read_directory_content();

    for article_name in article_names {
        match parse_one_article::markdown_to_styled_html(&article_name){
            Ok(_) => {
                println!("succesful parse")
            }
            Err(e) => {
                eprintln!("unsuccesful parse {}", e)
            }
        };
    }

}

