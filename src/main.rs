use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

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
        return;
    }

    if path.join(".git").is_dir() {
        display_git_repo(path);
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
    println!("{} (file)", path.display())
}

/// Display the mess in the git repository `path`.
///
/// * `path`: the path to the git repository
fn display_git_repo(path: &Path) {
    let mut mess_issues = Vec::new();

    let current_branch = get_current_git_branch(path);
    if let Some(branch) = current_branch {
        if branch != "main" && branch != "master" {
            mess_issues.push("not on default branch".to_string());
        }
    }

    let status_output = get_git_status(path);
    if !status_output.is_empty() {
        mess_issues.push("files added, modified, or removed".to_string());
    }

    let stash_count = get_stash_count(path);
    if stash_count > 0 {
        mess_issues.push("stash is not empty".to_string());
    }

    if !mess_issues.is_empty() {
        println!(
            "{} (git repository: {})",
            path.display(),
            mess_issues.join(", ")
        );
    }
}

/// Get the current git branch for a repository
///
/// * `path`: the path to the git repository
fn get_current_git_branch(path: &Path) -> Option<String> {
    let output = Command::new("git")
        .current_dir(path)
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Some(branch)
        }
        _ => None,
    }
}

/// Get the output of git status for a repository
///
/// * `path`: the path to the git repository
fn get_git_status(path: &Path) -> Vec<String> {
    let output = Command::new("git")
        .current_dir(path)
        .args(&["status", "--porcelain"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let status_text = String::from_utf8_lossy(&output.stdout);
            let mut result = Vec::new();

            for line in status_text.lines() {
                if !line.is_empty() {
                    // Extract the filename from the porcelain format output
                    // The format is "XY filename" where X and Y are status codes
                    let parts: Vec<&str> = line.splitn(2, ' ').collect();
                    if parts.len() == 2 {
                        result.push(parts[1].to_string());
                    }
                }
            }
            result
        }
        _ => Vec::new(),
    }
}

/// Get the number of stashed changes in a git repository
///
/// * `path`: the path to the git repository
fn get_stash_count(path: &Path) -> usize {
    let output = Command::new("git")
        .current_dir(path)
        .args(&["stash", "list"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let stash_text = String::from_utf8_lossy(&output.stdout);
            stash_text.lines().count()
        }
        _ => 0,
    }
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
