// use std::io;

use std::{fs::File, io::Read};

fn main() {
    // first arg is the filename
    let args = std::env::args().skip(1);

    let mut cliargs: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();
    let mut files_started = false;

    for arg in args {
        if !arg.starts_with("-") {
            files_started = true;
        }
        if !files_started {
            cliargs.push(arg);
            continue;
        }
        paths.push(arg);
    }
    println!("{}", cliargs.join(" "));
    println!("{}", paths.join(" "));

    for path in paths {
        let file = match File::open(&path) {
            Err(err) => {
                println!("file not working {}", err.to_string());
                continue;
            }
            Ok(file) => file,
        };

        output_file(file, cliargs.contains(&"-n".to_string()));
    }
}

fn output_file(mut file: File, enable_line_numbers: bool) {
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(err) => {
            println!("error reading file {}", err);
            return;
        }
        _ => {}
    }

    if !enable_line_numbers {
        println!("{contents}");
        return;
    }

    for (i, line) in contents.split("\n").enumerate() {
        let prefix = if enable_line_numbers { format!("     {i}  ") } else {String::new()};
        println!("{prefix}{line}");
    }
    return
}
