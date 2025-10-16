fn main() {
    let mut words = String::from("Hello world!");
    let w = first_word_index(&words.to_string());
    words.clear();

    // byte index 5 is out of bounds of ``
    let (first, _) = words.split_at(w);
    println!("{words} {first} {w}");
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
