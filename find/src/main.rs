use std::fs;
use std::env;
use std::path::Path;

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().unwrap();
    println!("search for \"{path}\"");
    let path = Path::new(&path);
    find(path)
}

fn find(p: &Path) {
    let res = fs::read_dir(p).unwrap();
    for item in res {
        let item = item.unwrap();
        let path = item.path();
        if path.is_dir() {
            find(&path);
        }
        let Some(path_name) = path.to_str() else {continue};
        println!("{path_name}");      
    }
}
