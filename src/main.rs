use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let mut args: Vec<String> = env::args().collect(); // First element is the executable path
    let no_extra_arg_provided = args.len() == 1;
    if no_extra_arg_provided {
        let default_arg = String::from("./");
        args.push(default_arg);
    }

    let ignore_patterns: Vec<String> = get_ignore_patterns();
    let show_arg: bool = args.len() > 2;
    for arg in &args[1..] {
        let expanded_arg = expand_arg(&arg);
        let path = Path::new(&expanded_arg);
        display(&path, &ignore_patterns, &arg, show_arg);
    }
}

/// Display the mess in `path`.
///
/// * `path`: the path of the mess to display
/// * `ignore_patterns`: the path patterns to ignore the mess
/// * `arg`: the CLI argument corresponding to the `path`
/// * `show_arg`: whether to show `arg` or not
fn display(path: &Path, ignore_patterns: &Vec<String>, arg: &String, show_arg: bool) {
    if path.is_dir() {
        display_dir(&path, &ignore_patterns, &arg, show_arg);
    } else if path.is_file() {
        display_file(&path, &ignore_patterns);
    } else {
        println!("no such file or directory: {}", arg)
    }
}

/// Display the mess in the directory `path`.
///
/// * `path`: the path of the mess to display
/// * `ignore_patterns`: the path patterns to ignore the mess
/// * `arg`: the CLI argument corresponding to the `path`
/// * `show_arg`: whether to show `arg` or not
fn display_dir(path: &Path, ignore_patterns: &Vec<String>, arg: &String, show_arg: bool) {
    if match_ignore_patterns(&path, &ignore_patterns) {
        return;
    }

    if show_arg {
        println!("{}:", arg);
    }

    if path.join(".messexclude").is_file() {
        println!("excluded: {}", path.display());
        return;
    }
    if path.join(".git/").is_dir() {
        println!("git repository: {}", path.display());
        return;
    }

    for child_path in fs::read_dir(&path).unwrap() {
        let child_path = child_path.unwrap().path();
        if child_path.is_dir() {
            display_dir(&child_path, &ignore_patterns, &arg, false);
        } else if child_path.is_file() {
            display_file(&child_path, &ignore_patterns);
        } else {
            println!("unknown element: {}", path.display())
        }
    }

    if show_arg {
        println!("")
    }
}

/// Display the mess in the file `path`.
///
/// * `path`: the path of the mess to display
/// * `ignore_patterns`: the path patterns to ignore the mess
fn display_file(path: &Path, ignore_patterns: &Vec<String>) {
    if match_ignore_patterns(&path, &ignore_patterns) {
        return;
    }

    println!("file: {}", path.display())
}

/// Expand the home directory symbol ("~") in the provided path.
///
/// * `arg`: the path argument
fn expand_arg(arg: &String) -> String {
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
    let ignore_file_path: String = expand_arg(&String::from("~/.messignore"));
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
fn match_ignore_patterns(path: &Path, ignore_patterns: &Vec<String>) -> bool {
    let path = path.to_str().unwrap();
    for ignore_pattern in ignore_patterns {
        if path.contains(&ignore_pattern.to_string()) {
            return true;
        }
    }
    return false;
}
