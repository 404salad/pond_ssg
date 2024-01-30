use std::fs::File;
use std::path::Path;
use std::thread;
use std::io::prelude::*;
use std::io::{Write, BufReader, Error};
use pulldown_cmark::{Parser, Options, html};

// parse markdown to html
fn parse_markdown(input: &str) -> String {
    let options = Options::empty();
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
// TODO parameterize and read from a config file
fn wrap_html(markdown_output: &str) -> String {
    let mut wrapped_html = String::from(markdown_output);
    wrapped_html="
<!DOCTYPE html>
    <html lang=\"en\">
    <head>
    <meta charset=\"UTF-8\">

    <link rel=\"stylesheet\" href=\"css/pico.yellow.min.css\">
    <title></title>
</head>
<body class=\"container\">
".to_string() + &wrapped_html +
"
</body>
</html>
";
    wrapped_html
}



pub fn markdown_to_styled_html<P: AsRef<Path>>(path: P) -> std::io::Result<()>{
    let html_from_md = parse_markdown(&read_markdown(path)?);
    let mut file = File::create("sample_output.html")?;
    let wrapped_html = wrap_html(&html_from_md);
    write!(file,"{wrapped_html}")?;
    Ok(())
}

