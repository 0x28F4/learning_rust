use std::{env, path::Path};

#[derive(Debug)]
struct Options {
    recursive: bool,
    force: bool,
    path_names: Vec<String>,
}

impl Options {
    fn new() -> Options {
        Options {
            recursive: false,
            force: false,
            path_names: Vec::new(),
        }
    }
}

fn main() {
    let mut options = Options::new();
    let args = env::args().skip(1);
    for arg in args {
        if arg.starts_with("-") {
            if arg == "-r" || arg == "-R" {
                options.recursive = true;
                continue;
            }
            if arg == "-f" || arg == "--force" {
                options.force = true;
            }

            if arg == "-rf" {
                options.recursive = true;
                options.force = true;
            }
        } else {
            options.path_names.push(arg);
        }
    }
    for path_name in options.path_names {
        let path: &Path = Path::new(&path_name);
        if !path.exists() {
            if !options.force {
                println!("rm: cannot remove '{path_name}': No such file or directory");
            }
            continue;
        }
        if path.is_dir() {
            if !options.recursive {
                println!("rm: cannot remove '{path_name}': Is a directory");
                continue;
            }
            std::fs::remove_dir_all(&path)
                .unwrap_or_else(|e| println!("rm: cannot remove '{path_name}': {e}"));
            continue;
        }
        std::fs::remove_file(&path)
            .unwrap_or_else(|e| println!("rm: cannot remove '{path_name}': {e}"));
    }
}
