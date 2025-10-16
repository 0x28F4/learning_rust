fn main() {
    let words = "Hello world!";
    let w = first_word_index(&words.to_string());
    let (first, _) = words.split_at(w);
    println!("{first}");
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