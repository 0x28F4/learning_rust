use rand::Rng;



#[derive(Debug)]
struct Jahr {
    value: u16
}


enum HartGeld {
    EinCent,
    ZweiCent,
    FuenfCent,
    ZehnCent,
    ZwanzigCent,
    Euro(Jahr),
    ZweiEuro,
}

impl HartGeld {
    fn value(&self) -> i32 {
        match self {
            HartGeld::EinCent => 1,
            HartGeld::ZweiCent => 2,
            HartGeld::FuenfCent => 5,
            HartGeld::ZehnCent => 10,
            HartGeld::ZwanzigCent => 20,
            HartGeld::Euro(v) => {println!("{v:?}");100},
            HartGeld::ZweiEuro => 200,
        }
    }
}

fn main() {
    let bag = vec![HartGeld::EinCent, HartGeld::ZwanzigCent, HartGeld::ZehnCent, HartGeld::Euro(Jahr { value: 2005 })];
    let total = bag.iter().map(|v|  v.value()).fold(0, |a, b| a + b);
    //let total: i32 = bag.iter().map(|v|  v.value()).sum();
    println!("total={}", total);

    option()
}

#[derive(Debug)]
struct V;

fn option() {
    let bag: Vec<Option<V>> = (0..).take(100).map(|_| {
        if rand::rng().random_bool(0.5) {
            Some(V)
        } else {
            None
        }
    }).collect();
    println!("{:#?}", bag);
}
