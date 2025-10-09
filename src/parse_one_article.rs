use super::config;
use maud::{html, Markup, DOCTYPE};
use pulldown_cmark::{html, Options, Parser};
use std::fs::File;
use std::io::{BufReader, Error, Read, Write};

// parse markdown to html
fn parse_markdown(input: &str) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn read_markdown(path: String) -> Result<String, Error> {
    let input = File::open(path)?;
    let mut buffered = BufReader::new(input);
    let mut markdown_input = String::new();
    buffered.read_to_string(&mut markdown_input)?;
    Ok(markdown_input)
}

fn wrap_html(markdown_output: &str, article: &str, user_config: &config::UserConfig) -> String {
    let page: Markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                link rel="stylesheet" href="../style.css";
                @if user_config.code_formatting {
                    link rel="stylesheet" href="../prism.css";
                }
                title { (article) }
            }
            body class="container" {
                a href="../index.html" { "home" }
                @if user_config.code_formatting {
                    script src="../prism.js" {}
                }
                h1 { (article) }
                hr;
                // raw HTML from markdown
                (maud::PreEscaped(markdown_output))
            }
        }
    };

    page.into_string()
}

/// convert a single .md file to html
pub fn markdown_to_styled_html(
    article: &str,
    user_config: &config::UserConfig,
) -> std::io::Result<()> {
    let input_path = String::from("content/") + article + ".md";
    let output_path = String::from("dist/articles/") + article + ".html";
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
        let html_output = parse_markdown(markdown_input);
        assert_eq!(html_output, "<h1>moon</h1>\n");
    }
}
