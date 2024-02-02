use std::fs;
use std::fs::File;
use std::io::{Write};

pub fn read_directory_content() -> Vec<String>{
    let mut article_names:Vec<String> = vec![];
    let paths_result = fs::read_dir("content");
    // iterating through paths
    match paths_result {
        Ok(paths) => {
            for path_result in paths {
                match path_result {
                    Ok(path) => {
                        if let Some(path_str) = path.path().to_str() {
                            article_names.push(path_str
                                               .trim_start_matches("content/")
                                               .trim_end_matches(".md")
                                               .to_string());
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

pub fn create_homepage() -> std::io::Result<()>{
    let article_names = read_directory_content();
    let output_path =String::from("dist/index.html");
    let mut document=format!("
<!DOCTYPE html>
    <html lang=\"en\">
    <head>
    <meta charset=\"UTF-8\">

    <link rel=\"stylesheet\" href=\"pico.min.css\">
    <title>homepage</title>
</head>
<br>
<h1>HOMEPAGE</h1>
<body class=\"container\">
<br>
<br>
");
    for article_name in article_names {
        document.push_str(format!("
<article> 
    <a href=\"articles/{article_name}.html\"> {article_name} </a>
</article>
        ").as_str()
    );
    }
    document.push_str("
</body>
</html>
    ");
    let mut file = File::create(output_path)?;
    write!(file,"{document}")?;
    Ok(())
}

