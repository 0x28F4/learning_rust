fn main() {
    let mut s = "123";

    s = "321";
    let words = String::from("Hello world!");

    let first = first_word(&words);

    // this doesnt work anymore
    // words.clear();
    println!("{first}");
}

fn first_word(s: &str) -> &str {
    for (i, &sb) in s.as_bytes().iter().enumerate() {
        if sb == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

