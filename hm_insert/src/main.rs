use std::collections::HashMap;

#[derive(Debug)]
enum Foo {
    I32(i32),
    U32(u32),
    Text(String),
}

#[derive(Debug)]
struct Bar {
    name: String,
    value: Foo,
}

fn main() {
    let mut h = HashMap::new();
    let v1 = Foo::I32(0);
    let b1 = Bar {
        name: "b1".to_string(),
        value: v1,
    };
    h.insert(1, b1);

    let v = h.get(&1);
    match v {
        Some(bar) => {
            println!("bar: {:?}", bar);
        }
        None => {
            println!("not found");
        }
    }

    println!("update in progress");
    update(&mut h);
    let v = h.get(&1);
    match v {
        Some(bar) => {
            println!("bar: {:?}", bar);
        }
        None => {
            println!("not found");
        }
    }
}

fn update(h: &mut HashMap<i32, Bar>) {
    if let Some(x) = h.get_mut(&1) {
        x.value = Foo::Text("hello".to_string());
    }
}
