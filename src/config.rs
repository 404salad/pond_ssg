/*
 * instead of setting initial config this way,
 * just create a template file which the user can open and fill in
 * create a --config thing so that user can override file name if needed
 * */
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Deserialize, Clone, Serialize)]
pub struct UserConfig {
    pub author_name: String,
    pub blog_name: String,
    pub code_formatting: bool, // Set to false by default
}

impl fmt::Display for UserConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "author name > {}
blog name > {}
code formatting > {}
",
            self.author_name, self.blog_name, self.code_formatting
        )
    }
}

pub struct CommandLineArgs {
    pub watcher: bool,
    pub help: bool,
    pub silent: bool,
}

pub fn initial_setup() -> anyhow::Result<()> {
    create_project_files()?;

    // taking user input
    print!("enter your name > ");
    io::stdout().flush()?;

    let mut author_name = String::new();
    io::stdin().read_line(&mut author_name)?;

    print!("enter blog name > ");
    io::stdout().flush()?;

    let mut blog_name = String::new();
    io::stdin().read_line(&mut blog_name)?;

    let user = UserConfig {
        author_name,
        blog_name,
        code_formatting: false, // set to false
    };

    let toml_config = toml::to_string(&user).context("invalid config file")?;

    // writing to config file
    let config_path = Path::new("config.toml");

    let mut config_file = fs::File::create(config_path).context("cant create config file")?;

    config_file
        .write_all(toml_config.as_bytes())
        .context("cant write to config file")?;

    println!("successfully created config file");

    Ok(())
}

pub fn read_config() -> Result<UserConfig, toml::de::Error> {
    let config_file_data = fs::read_to_string("config.toml").unwrap_or_else(|_| String::new());

    let config: UserConfig = toml::from_str(&config_file_data)?;

    Ok(config)
}

pub fn config_exists() -> bool {
    Path::new("config.toml").exists()
}

fn create_project_files() -> io::Result<()> {
    fs::create_dir_all("content")?;
    fs::create_dir_all("dist")?;
    fs::create_dir_all("dist/articles")?;
    let main_css = include_str!("../styling_deps/style.css");
    fs::write("dist/style.css", main_css)?;
    Ok(())
}

pub fn read_cl_args() -> CommandLineArgs {
    let args = env::args().skip(1);
    let mut watcher = false;
    let mut help = false;
    let mut silent = false;
    for arg in args {
        if arg.trim().starts_with("--watch") {
            watcher = true;
        } else if arg.trim().starts_with("--help") {
            help = true;
        } else if arg.trim().starts_with("--silent") {
            silent = true;
        }
    }
    CommandLineArgs {
        watcher,
        help,
        silent,
    }
}
