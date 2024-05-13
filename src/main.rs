pub mod parse_one_article;
pub mod consolidate_into_homepage;
pub mod config;

use std::thread;
use std::time::Duration;
use std::env;

use console::Term;

fn main() {

    /*
    let args: Vec<String> = env::args().collect();
    let opt = &args[1]; // prune 
    */

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

    // TODO: only run this function if config toml doesnt exist 
    //config::initial_config();

    let user_config = config::read_config().unwrap();
    println!("{}",user_config);
    
    // Remove previous content
    let demo_dir = fs::read_dir("/path/to/dir");
    delete_dir_contents(demo_dir);

    let article_names = consolidate_into_homepage::read_directory_content();
    println!("{:?}", article_names);
    // rebuilding all the articles in content directory
    for article_name in article_names {
        match parse_one_article::markdown_to_styled_html(&article_name, &user_config){
            Ok(_) => {
                println!("succesful parse")
            }
            Err(e) => {
                eprintln!("unsuccesful parse {}", e)
            }
        };
    }
    match consolidate_into_homepage::create_homepage(&user_config) {
            Ok(_) => {
                println!("succesfully created homepage")
            }
            Err(e) => {
                eprintln!("unsuccesful in creating homepage {}", e)
            }
    };


}

