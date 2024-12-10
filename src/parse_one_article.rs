use super::config;
use pulldown_cmark::{html, Options, Parser};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, Write};
use std::path::Path;

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
fn wrap_html(markdown_output: &str, article: &str, _user_config: &config::UserConfig) -> String {
    let mut wrapped_html = String::from(markdown_output);
    wrapped_html = format!(
        "
<!DOCTYPE html>
    <html lang=\"en\">
    <head>
    <meta charset=\"UTF-8\">
    <link rel=\"stylesheet\" href=\"../style.css\">
    <link rel=\"stylesheet\" href=\"../prism.css\">
    <title>{article}</title>
</head>
<body class=\"container\">
<a href='../index.html'>home</a>
<script src=\"../prism.js\"></script>
<h1>  {article}  </h1>
<hr>
"
    ) + &wrapped_html
        + "
</body>
</html>
";
    wrapped_html
}

/// convert a single .md file to html
pub fn markdown_to_styled_html(
    article: &str,
    user_config: &config::UserConfig,
) -> std::io::Result<()> {
    println!("parsing - {article}");
    let mut input_path = String::from("content/") + &article.to_owned();
    let mut output_path = String::from("dist/articles/") + &article.to_owned();
    input_path.push_str(".md");
    output_path.push_str(".html");
    let html_from_md = parse_markdown(&read_markdown(input_path)?);
    let mut file = File::create(output_path)?;
    let wrapped_html = wrap_html(&html_from_md, article, user_config);
    write!(file, "{wrapped_html}")?;
    Ok(())
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown() {
        let markdown_input = "# moon";
        let html_output = parse_markdown(&markdown_input);
        assert_eq!(html_output, "<h1>moon</h1>\n");
    }
}
