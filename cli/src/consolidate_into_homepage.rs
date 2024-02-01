use std::fs;

pub fn list_paths() {
    // TODO use proper error handling
    let paths = fs::read_dir("content").unwrap();
    for path in paths {
        // TODO use proper error handling
        println!("{}", path.unwrap().path().display());
    }

}
