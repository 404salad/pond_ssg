use super::config;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn read_directory_content() -> Vec<String> {
    let mut article_names: Vec<String> = vec![];
    let paths_result = fs::read_dir("content");
    // iterating through paths
    match paths_result {
        Ok(paths) => {
            for path_result in paths {
                match path_result {
                    Ok(path) => {
                        if let Some(path_str) = path.path().to_str() {
                            article_names.push(
                                path_str
                                    .trim_start_matches("content/")
                                    .trim_end_matches(".md")
                                    .to_string(),
                            );
                        } else {
                            eprintln!("Error converting path to string");
                        }
                    }
                    Err(err) => {
                        eprintln!("Error reading directory entry: {}", err);
                        continue;
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
        }
    };
    article_names
}

pub fn create_homepage(user_config: &config::UserConfig) -> std::io::Result<()> {
    let article_names = read_directory_content();
    let output_path = String::from("dist/index.html");

    let mut document = String::new();
    document.push_str(&format!(
            "<!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"UTF-8\">
            <title>{}</title>
            <link rel=\"stylesheet\" href=\"style.css\">
            <script>
                function filterArticles() {{ 
                    var input, filter, articleList, articles, article, title, i, titleText;
                    input = document.getElementById('search');
                    filter = input.value.toUpperCase();
                    articleList = document.getElementById('articleList');
                    articles = articleList.getElementsByTagName('article');

                    // Loop through all articles, and hide those whose titles don't match the search query
                    for (i = 0; i < articles.length; i++) {{
                        article = articles[i];
                        title = article.getElementsByTagName('a')[0];
                        titleText = title.textContent || title.innerText;
                        if (titleText.toUpperCase().indexOf(filter) > -1) {{
                            article.style.display = '';
                        }} else {{
                            article.style.display = 'none';
                        }}
                    }}
                }}
            </script>
        </head>
        <body class=\"container\">
            <br>
            <h1>{}</h1>
            <h6>{}</h6>
            <input type=\"search\" id=\"search\" name=\"search\" placeholder=\"Type to search...\" onkeyup=\"filterArticles()\">
            <br>
            <br>
            <section id=\"articleList\">",
            user_config.blog_name, user_config.blog_name, user_config.author_name
                ));

    for article_name in article_names {
        document.push_str(&format!(
            "<article>
                <a href=\"articles/{article_name}.html\"> {article_name} </a>
            </article>",
            article_name = article_name
        ));
    }

    document.push_str(
        "
            </section>
        </body>
        </html>",
    );

    let mut file = File::create(output_path)?;
    write!(file, "{}", document)?;
    Ok(())
}
