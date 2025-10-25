struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

fn main() {
    let mut user = build_user(
        String::from("my-first-user"),
        String::from("foo@example.com"),
    );

    user.username = String::from("foo");
    println!("Hello, world! {}", user.username);

    let user2 = User {
        ..user
    };

    println!("hi {}", user2.username);
    // wont work because user2 now is owner of the struct
    // println!("hi {}", user.username); 


    interesting_structs();
    methods();
    lifetime_struct();
}


struct Color(i32, i32,i32);
struct Point(i32, i32,i32);
struct AlwaysEqual;


struct LifetimeStruct<'a> {
    field: &'a str,
}

fn interesting_structs() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    let alt_origin = Point{0: 0, 1: 0, 2: 0};
    let eq = AlwaysEqual;
    let other_eq = AlwaysEqual{..eq};



    let novel = &String::from("Call me Ishmael. Some years ago...");
    let i = LifetimeStruct { field: novel };
}


#[derive(Debug)]
struct rect {
    w: u32,
    h: u32,
}

impl rect {
    fn area(&self) -> u32 {
        dbg!(self.h * self.w)
    }
}

fn methods() {
    let r = rect {
        w: 20,
        h: 30,
    };

    dbg!(&r);
    println!("rect {r:?} area is {}",  r.area());
}

// what about lifetimes 

#[derive(Debug)]
struct cooker<'a> {
    bra: &'a str,
    bro: String
}

impl<'a> cooker<'a> {
    fn upper(&'a self)->String {
        self.bra.to_uppercase()
    }

    fn upper_apply(&'a mut self)->() {
        self.bro = self.bro.to_uppercase();
    }

}

fn lifetime_struct()  {
    let mut c = cooker{bra: &String::from("hello world!"), bro: String::from("foobar")};
    println!("upper: {}", c.upper());
    c.upper_apply();

    // println!("{c:?}");
}