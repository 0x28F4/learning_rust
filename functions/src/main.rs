fn main() {
    println!("Hello, world!");
    println!("{}", foo(3));

    let y = {
        let x = 3;
        x + 1
    };

    println!("{y}");

    let inlinefn = || {
        555
    };

    println!("{}", inlinefn());
    let otherfn = simple;
    println!("{}", otherfn());

    // control flow

    let num = if 3 < 4 {7} else {6};

    // no work, because different types in branches
    // let number = if true { 5 } else { "7" };
    println!("{}", num);

    let ret = loop {
        println!("hi");
        // return in loop
        break "lol"
    };

    println!("{ret}");


    // nested loop
    let mut k = 1;
    let ret = 'outer: loop {
        k *= 3;
        loop {
            if k % 2 == 0 {
                break 'outer k
            }
            k += 1;
            break;
        }
    };
    println!("{ret}");


    let mut a = 20;

    let _ret = while a > 10 {
        // Language grammar/semantics: loop is defined as an expression that yields a value when broken with break <expr>. while/for were designed as control-flow statements that evaluate to unit (), so the compiler rejects assigning them a non-unit value.
        // break 7
        a -= 4
    };


    let iter = (1..=4).rev();
    for i in iter {
        println!("loop {i}");
    }
}


fn simple() -> i32 {
    42
}

fn foo(i: i32) -> i32 {
    println!("{i}");
    i
}