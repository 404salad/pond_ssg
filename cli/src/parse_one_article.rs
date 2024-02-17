use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::{Write, BufReader, Error};
use pulldown_cmark::{Parser, Options, html};
use super::config;

// parse markdown to html
fn parse_markdown(input: &str) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn read_markdown<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let input = File::open(path)?;
    let mut buffered = BufReader::new(input);
    let mut markdown_input = String::new();
    buffered.read_to_string(&mut markdown_input)?;
    Ok(markdown_input)
}

// wrapper for input so that standard html and styles can be injected after converting to html
fn wrap_html(markdown_output: &str, article: &str, user_config: &config::UserConfig) -> String {
    let mut wrapped_html = String::from(markdown_output);
    wrapped_html=format!("
<!DOCTYPE html>
    <html lang=\"en\">
    <head>
    <meta charset=\"UTF-8\">

    <link rel=\"stylesheet\" href=\"../css/pico.{}.min.css\">
    <title>{article}</title>
</head>
<body class=\"container\">
<a href='../index.html'>home</a>
<br>
<h1> | {article} | </h1>
<hr>
", user_config.accent_color
).to_string() + &wrapped_html +
"
</body>
</html>
";
    wrapped_html
}

/// convert a single .md file to html
pub fn markdown_to_styled_html(article: &str, user_config: &config::UserConfig) -> std::io::Result<()>{
    let mut input_path = String::from("content/") + &article.to_owned();
    let mut output_path =String::from("dist/articles/")+ &article.to_owned();
    input_path.push_str(".md");
    output_path.push_str(".html");
    println!("{input_path} => {output_path}");
    let html_from_md = parse_markdown(&read_markdown(input_path)?);
    let mut file = File::create(output_path)?;
    let wrapped_html = wrap_html(&html_from_md, article, user_config);
    write!(file,"{wrapped_html}")?;
    Ok(())
}

