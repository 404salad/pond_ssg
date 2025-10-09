use crate::config;
use crate::file_utils::read_generated_filepaths;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use std::fs::File;
use std::io::Write;

pub fn create_homepage(user_config: &config::UserConfig) -> std::io::Result<()> {
    let article_names = read_generated_filepaths();
    let output_path = "dist/index.html";

    let document: Markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                title { (user_config.blog_name) }
                link rel="stylesheet" href="style.css";
                script {
                    (PreEscaped(r#"
                        function filterArticles() {
                            var input, filter, articleList, articles, article, title, i, titleText;
                            input = document.getElementById('search');
                            filter = input.value.toUpperCase();
                            articleList = document.getElementById('articleList');
                            articles = articleList.getElementsByTagName('article');

                            for (i = 0; i < articles.length; i++) {
                                article = articles[i];
                                title = article.getElementsByTagName('a')[0];
                                titleText = title.textContent || title.innerText;
                                if (titleText.toUpperCase().indexOf(filter) > -1) {
                                    article.style.display = '';
                                } else {
                                    article.style.display = 'none';
                                }
                            }
                        }
                    "#))
                }
            }
            body class="container" {
                br;
                h1 { (user_config.blog_name) }
                h6 { (user_config.author_name) }
                input type="search" id="search" name="search" placeholder="Type to search..." onkeyup="filterArticles()";
                br; br;
                section id="articleList" {
                    @for article in &article_names {
                        article {
                            a href=("articles/".to_owned() + article) {
                                (article)
                            }
                        }
                    }
                }
            }
        }
    };

    let mut file = File::create(output_path)?;
    write!(file, "{}", document.into_string())?;
    Ok(())
}
