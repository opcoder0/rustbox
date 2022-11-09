// use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug)]
struct Foo {
    name: String,
    value: i32,
}

fn main() {
    let mut minions: HashMap<i32, &Foo> = HashMap::new();

    let aga = Foo {
        name: "Aga".to_string(),
        value: 8395,
    };
    minions.insert(1, &aga);

    let eep = Foo {
        name: "Eep".to_string(),
        value: 8905,
    };
    minions.insert(2, &eep);

    println!("minions added:");
    for (k, v) in minions.iter() {
        println!("k: {}, v: {:?}", k, v);
    }

    println!("am not eep. i am eek");
    {
        if let Some(v) = minions.get_mut(&2) {
            *v = &Foo {
                name: "eek".to_string(),
                value: 8905,
            };
        }
    }

    println!("i fixed your name eek:");
    for (k, v) in minions.iter() {
        println!("k: {}, v: {:?}", k, v);
    }
}
