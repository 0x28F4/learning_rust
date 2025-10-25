use std::fs;
use std::env;
use std::path::Path;

fn main() {
    let mut args = env::args().skip(1);
    let query = args.next().unwrap();
    println!("search for \"{query}\"");

    let res = fs::read_dir(".").unwrap();
    for item in res {
        let item = item.unwrap();
        let path_name = item.path();
        let Some(path_name) = path_name.to_str() else {continue};
        if path_name.contains(&query) {
            println!("{path_name}");      
        }
    }
}


fn find(query: &str, p: &Path) {
    let res = fs::read_dir(p).unwrap();
    for item in res {
        let item = item.unwrap();
        let path = item.path();

        if path.is_dir() {
            find(&query, &path);
            continue;
        }
        let Some(path_name) = path.to_str() else {continue};
        if path_name.contains(&query) {
            println!("{path_name}");      
        }
    }
}
