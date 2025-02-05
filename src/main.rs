use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let arg = &args[1]; // Index 0 corresponds to debuggging stuff
        display_dir_content(&Path::new(&arg));
    } else {
        display_dir_content(&Path::new("./"));
    }
}

fn display_dir_content(path: &Path) {
    if path.is_dir() == false {
        println!("not a dir: {}", path.display());
        return;
    }

    if path.join(".git/").is_dir() == true {
        println!("git repo: {}", path.display());
        return;
    }

    for child_path in fs::read_dir(&path).unwrap() {
        let child_path = child_path.unwrap().path();
        if child_path.is_dir() == true {
            display_dir_content(&child_path);
        } else {
            println!("file: {}", child_path.display())
        }
    }
}
