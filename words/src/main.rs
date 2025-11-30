fn main() {
    let words = String::from("Hello world!");

    let first = first_word(&words);

    // this doesnt work anymore
    // words.clear();
    println!("{first}");


    let lhs = String::from("lhs");
    let rhs = String::from("rhs");
    let out = pick(&lhs, &rhs, true);

    println!("{out}");

    wtf();
}

fn first_word(s: &str) -> &str {
    for (i, &sb) in s.as_bytes().iter().enumerate() {
        if sb == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn pick<'a>(a: &'a str, b: &'a str, pick_a: bool) -> &'a str {
    if pick_a {
        return &a;
    }
    return b;
}



fn wtf() {
    let v = String::from("1");
    let longest = |b: & str| -> & str { &b[..] };

    let l = longest(&v);
    println!("{l}");
}