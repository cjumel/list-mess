use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..]; // Discard first argument, the path of the executable

    if args.len() == 0 {
        let default_path = Path::new("./");
        display_dir_content(&default_path);
    } else {
        for arg in args {
            let path = Path::new(arg);
            if path.is_dir() == false {
                println!("{} is not a directory\n", path.display());
                continue;
            }

            println!("mess in {}:", path.display());
            display_dir_content(&path);
            println!("");
        }
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
