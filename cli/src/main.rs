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

    consolidate_into_homepage::list_paths();
    match parse_one_article::markdown_to_styled_html("sample"){
        Ok(_) => {
            println!("succesful parse")
        }
        Err(e) => {
            eprintln!("unsuccesful parse {}", e)
        }
    };
    /*
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    */


}

