fn main() {
    simple();
    borrowing();
}


fn borrowing() {
    let calculate_len = |x: &String| -> usize {
        x.len()
    };

    let s1 = String::from("hi");
    let lenofs1 = calculate_len(&s1);

    println!("{s1} - {lenofs1}");


    // change change a mutable reference
    // let change = |x: &String|{
    //     x.push_str(" string")
    //     ^ `x` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // };


    // mutable references
    let change = |x: &mut String| {
        x.push_str(" world");
    };
    let mut a = String::from("hi");
    change(&mut a);
    println!("{a}");


    // assign mutable reference twice
    // let mut s = String::from("foo");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{r1} {r2}");
        //     error[E0499]: cannot borrow `s` as mutable more than once at a time
        //   --> src/main.rs:38:14
        //    |
        // 37 |     let r1 = &mut s;
        //    |              ------ first mutable borrow occurs here
        // 38 |     let r2 = &mut s;
        //    |              ^^^^^^ second mutable borrow occurs here
        // 39 |
        // 40 |     println!("{r1} {r2}");
        //    |                -- first borrow later used here


    // dangle (?)
    // let other = {
    //     let s = String::from("123");
    //     &s
    // };
    // println!("{other}")
        // error[E0597]: `s` does not live long enough
        //   --> src/main.rs:55:9
        //    |
        // 53 |     let other = {
        //    |         ----- borrow later stored here
        // 54 |         let s = String::from("123");
        //    |             - binding `s` declared here
        // 55 |         &s
        //    |         ^^ borrowed value does not live long enough
        // 56 |     };
        //    |     - `s` dropped here while still borrowed
}


// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
// error[E0106]: missing lifetime specifier
//   --> src/main.rs:71:16
//    |
// 71 | fn dangle() -> &String {
//    |                ^ expected named lifetime parameter
//    |
//    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
//    |
// 71 | fn dangle() -> &'static String {

fn simple() {
    let s1 = String::from("hello");

    let s2 = {
        s1
    };

    // no work, because s1 invalidated
    // println!("{s1}");
    println!("{s2}");
    

    ident(s2);
    // no work because s2 invalidated
    // println!(s2);


    let r1 = String::from("hi");
    let mut r2 = String::from("");
    for c in r1.chars() {
        r2.push(c);
    }
    println!("{r1}");

}

fn ident(x: String) -> String {
    x
} 