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

    let enable_line_numbers = cliargs.iter().any(|s| {s == "-n"});
    for path in paths {
        match File::open(&path) {
            Err(err) => {
                println!("file not working {}", err.to_string());
                continue;
            }
            Ok(mut file) => output_file(&mut file, enable_line_numbers),
        };
    }
}

fn output_file(file: &mut File, enable_line_numbers: bool) {
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

    for (i, line) in contents.lines().enumerate() {
        let prefix = if enable_line_numbers { format!("     {i}  ") } else {String::new()};
        println!("{prefix}{line}");
    }
    return
}
