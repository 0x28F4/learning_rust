use std::panic;

fn main() {
    println!("integer wrapping");
    wrapping();
    println!("tuples");
    tuples();
    println!("arrays");
    arrays();
}

fn wrapping() {
    let guess: i128 = "42".parse().expect("Not a number!");
    println!("{guess} {}", size_of::<i8>());

    let mut a: i8 = 0;
    loop {
        // a = a.wrapping_add(1);
        a = match a.checked_add(1) {
            Some(n) => n,
            None => {
                println!("overflow");
                break;
            }
        };
        print!("{a} ");
        if a == 0 {
            break;
        }
    }
}

fn tuples() {
    let tup = (500, 6.4, 1);
    println!("{}", tup.0);

    let empty = ();
}

fn arrays() {
    let a = [1, 2, 3, 4, 5, 100];
    println!("{}", a[0]);

    let typed_a: [i64; 1] = [1];

    for i in 1..3 {
        let result = panic::catch_unwind(|| typed_a[i]);

        match result {
            Ok(value) => println!("{value}"),
            Err(_) => println!("caught panic"),
        };
    }
    for i in 1..3 {
        match typed_a.get(i) {
            Some(n) => println!("got {n}"),
            None => println!("nope"),
        }
    }
    
}
