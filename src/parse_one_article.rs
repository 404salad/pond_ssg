use super::config;
use maud::{html, Markup, DOCTYPE};
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read, Write};
use toml::value::Datetime;

#[derive(Debug, Deserialize)]
struct Metadata {
    title: String,
    tags: Vec<String>,
    date: Datetime,
}

// parse markdown to html
fn parse_markdown(input: &str) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn read_file_and_metadata(path: String) -> anyhow::Result<(Metadata, String), anyhow::Error> {
    // TODO: efficiency
    let mut lines: Vec<String> = std::fs::read_to_string(&path)
        .expect("lines conversion failed")
        .lines()
        .map(String::from)
        .collect();

    if lines.len() < 5 {
        return Err(anyhow::anyhow!("add metadata to the file"));
    }
    let (metadata_a, markdown_blog_a) = lines.split_at_mut(5);
    let markdown_blog = markdown_blog_a.join("\n");
    // TODO: this is so hacky fix it maybe
    // also make it have a version for backwards compatability maybe
    let meta = &metadata_a[1..4].join("\n");

    let metadata: Metadata = match toml::from_str::<Metadata>(meta) {
        Ok(m) => m,
        Err(e) => {
            eprintln!(
                r#"
metdata error, have this on the top of each file in src/{path}
```toml
title = "title for the article"
tags = ["tag1","tag2"] // can be empty also
date = 2025-04-20
```
                "#
            );
            return Err(e.into());
        }
    };

    dbg!(&metadata);

    //Ok((meta, markdown_input))
    Ok((metadata, markdown_blog))
}

fn wrap_html(
    markdown_output: &str,
    meta: Metadata,
    article: &str,
    user_config: &config::UserConfig,
) -> String {
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
                h1 { (meta.title) }
                @if !meta.tags.is_empty() {
                    "tags: "
                        @for tag in &meta.tags {
                            span { (tag) }" "
                        }
                }
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
) -> anyhow::Result<(), anyhow::Error> {
    let input_path = String::from("content/") + article + ".md";
    let output_path = String::from("dist/articles/") + article + ".html";
    let (metadata, file_data) = read_file_and_metadata(input_path)?;

    //let tags_from_md: Vec<String> = parse_tags(input_path);

    let html_from_md = parse_markdown(&file_data);
    let mut file = File::create(output_path)?;
    let wrapped_html = wrap_html(&html_from_md, metadata, article, user_config);
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
