fn main() {
    let words = String::from("Hello world!");

    let first = &words[..5];
    let second = &words[6..];

    println!("{first},{second}")
}

