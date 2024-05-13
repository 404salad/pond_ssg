use std::fmt;
use serde::Deserialize;
use std::io::{self,Write};
use std::fs::read_to_string;


#[derive(Deserialize)]
pub struct UserConfig {
    pub author_name: String,
    pub blog_name: String,
}

impl fmt::Display for UserConfig {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "author name > {} \nblog name > {}
", self.author_name, self.blog_name)
    }
}

/*
pub fn initial_config () {
    print!("enter your name > " );
    io::stdout().flush().expect("flush failed");
    let mut author_name = String::new();
    io::stdin()
        .read_line(&mut author_name)
        .expect("readline failed");

    print!("enter blog name > " );
    io::stdout().flush().expect("flush failed");
    let mut blog_name = String::new();
    io::stdin()
        .read_line(&mut blog_name)
        .expect("readline failed");

    let user = UserConfig{
        author_name,
        blog_name,
    };
}
*/

pub fn read_config() -> Result<UserConfig, toml::de::Error>{
    let config_file_data = match read_to_string("config.toml"){
        Ok(d) => {d},
        Err(_) => {String::new()}
    };

    let config: UserConfig = toml::from_str(&config_file_data)?;
        
    Ok(config)

}

