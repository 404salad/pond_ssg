pub mod parse;
use std::thread;
use std::time::Duration;

use console::Term;

fn main(){
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
    term.write_line("A simple cli tool to convert markdown to blog");
    let _ = parse::markdown_to_styled_html("sample_input");
    /*
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    */


}

