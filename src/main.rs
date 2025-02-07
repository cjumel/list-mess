use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..]; // Discard first argument, the path of the executable

    if args.len() == 0 {
        let default_path = Path::new("./");
        display_dir_content(&default_path, false);
    } else {
        for arg in args {
            let arg = expand_arg_path(arg);
            let path = Path::new(&arg);
            display_dir_content(&path, true);
        }
    }
}

/// Expand the home directory symbol ("~") in the provided path.
///
/// * `arg`: the path argument
fn expand_arg_path(arg: &String) -> String {
    if arg.starts_with("~/") {
        return [
            // WARN: this won't work on Windows due to `env::home_dir()`
            #[allow(deprecated)]
            env::home_dir().unwrap().to_str().unwrap(),
            arg.strip_prefix("~").unwrap(),
        ]
        .join("");
    } else {
        return String::from(arg);
    }
}

/// Display the content of a directory.
///
/// * `path`: the path of the directory
/// * `show_path`: if true, display the path of the directory before showing its mess
fn display_dir_content(path: &Path, show_path: bool) {
    if path.is_dir() == false {
        println!("{} is not a directory", path.display());
        return;
    }
    if show_path == true {
        println!("mess in {}:", path.display());
    }

    if path.join(".messexclude").is_file() == true {
        println!("excluded: {}", path.display());
        return;
    }
    if path.join(".git/").is_dir() == true {
        println!("git repository: {}", path.display());
        return;
    }

    for child_path in fs::read_dir(&path).unwrap() {
        let child_path = child_path.unwrap().path();
        if child_path.is_dir() == true {
            display_dir_content(&child_path, false);
        } else {
            println!("file: {}", child_path.display())
        }
    }
}
