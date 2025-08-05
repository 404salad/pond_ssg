/*
 * instead of setting inital config this way, 
 * just create a template file which the user can open and fill in 
 * create a --config thing so that user can override file name if needed
 * */
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{read_to_string, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Deserialize, Clone, Serialize)]
pub struct UserConfig {
    pub author_name: String,
    pub blog_name: String,
}

impl fmt::Display for UserConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "author name > {} \nblog name > {}
",
            self.author_name, self.blog_name
        )
    }
}

pub fn initial_config() {
    // only run config setup if the config file doesnt exists
    if Path::new("config.toml").exists() {
        return;
    }

    // taking user input
    print!("enter your name > ");
    io::stdout().flush().expect("flush failed");
    let mut author_name = String::new();
    io::stdin()
        .read_line(&mut author_name)
        .expect("readline failed");

    print!("enter blog name > ");
    io::stdout().flush().expect("flush failed");
    let mut blog_name = String::new();
    io::stdin()
        .read_line(&mut blog_name)
        .expect("readline failed");

    let user = UserConfig {
        author_name,
        blog_name,
    };
    let toml_config: String = match toml::to_string(&user) {
        Ok(config_str) => config_str,
        Err(why) => panic!("invalid config file due to {why}"),
    };

    // writing to config file
    let config_path = Path::new("config.toml");

    let mut config_file = match File::create(&config_path) {
        Err(why) => panic!("cant create config file because {}", why),
        Ok(file) => file,
    };

    match config_file.write_all(toml_config.as_bytes()) {
        Err(why) => panic!("cant write to config file because {}", why),
        Ok(_) => println!("succesfully created config file"),
    }
}

pub fn read_config() -> Result<UserConfig, toml::de::Error> {
    let config_file_data = match read_to_string("config.toml") {
        Ok(d) => d,
        Err(_) => String::new(),
    };

    let config: UserConfig = toml::from_str(&config_file_data)?;

    Ok(config)
}
