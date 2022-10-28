use std::any::Any;
use std::string::ToString;

struct Foo {
    name: String,
    value: Option<Box<dyn Any>>,
}

fn main() {
    let foo = Foo {
        name: "one".to_string(),
        value: Some(Box::new(123)),
    };
    let _t = get_value::<i32>(&foo);
    println!("got value back: {}", _t);
}

fn get_value<T: Clone + 'static>(foo: &Foo) -> Box<T> {
    let v = foo.value.as_ref().expect("cannot find value");
    let t = v.downcast_ref::<T>();
    let t = t.expect("cannot extract from option");
    let t = t.clone();
    return Box::new(t);
}
