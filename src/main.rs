use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..]; // Discard first argument, the path of the executable

    let ignore_patterns: Vec<String> = get_ignore_patterns();

    if args.len() == 0 {
        let default_path = Path::new("./");
        display_dir_content(&default_path, false, &ignore_patterns);
    } else {
        for arg in args {
            let arg = expand_arg_path(arg);
            let path = Path::new(&arg);
            display_dir_content(&path, true, &ignore_patterns);
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

/// Read the `~/.messignore` file and output the patterns in it. This will output the lines
/// contained in this file, except those empty or starting with "#".
fn get_ignore_patterns() -> Vec<String> {
    let ignore_file_path: String = expand_arg_path(&String::from("~/.messignore"));
    let ignore_file_path: &Path = Path::new(&ignore_file_path);
    if ignore_file_path.is_file() {
        let content = fs::read_to_string(&ignore_file_path).unwrap();
        let mut content_lines: Vec<String> = Vec::new();
        for content_line in content.split("\n") {
            if content_line == "" {
                continue;
            }
            if content_line.starts_with("#") {
                continue;
            }
            content_lines.push(String::from(content_line));
        }
        return Vec::from_iter(content_lines);
    } else {
        return Vec::new();
    }
}

/// Output whether the given `path` matches one of the `ignore_patterns`. The path is not expanded
/// if it is relative, so this will only work reliably for file names.
///
/// * `path`: the path to check
/// * `ignore_patterns`: the ignore patterns to check the path with
fn match_ignore_pattern(path: &Path, ignore_patterns: &Vec<String>) -> bool {
    let path = path.to_str().unwrap();
    for ignore_pattern in ignore_patterns {
        if path.contains(&ignore_pattern.to_string()) {
            return true;
        }
    }
    return false;
}

/// Display the content of a directory.
///
/// * `path`: the path of the directory
/// * `show_path`: if true, display the path of the directory before showing its mess
fn display_dir_content(path: &Path, show_path: bool, ignore_patterns: &Vec<String>) {
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
            display_dir_content(&child_path, false, &ignore_patterns);
        } else {
            if !match_ignore_pattern(&child_path, &ignore_patterns) {
                println!("file: {}", child_path.display())
            }
        }
    }
}
