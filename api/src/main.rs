use std::str::FromStr;

trait FooTrait {
    type ValueType;
    fn geta(&self) -> &String;
    fn getb(&self) -> i32;
    fn getc(&self) -> Option<&Self::ValueType>;
}

#[derive(Debug)]
struct Foo<T> {
    a: String,
    b: i32,
    c: Option<T>,
}

impl<T> FooTrait for Foo<T> {
    type ValueType = T;

    fn geta(&self) -> &String {
        &self.a
    }

    fn getb(&self) -> i32 {
        self.b
    }

    fn getc(&self) -> Option<&T> {
        self.c.as_ref()
    }
}

impl<T: 'static> Foo<T> {
    fn new(a: String, b: i32, c: Option<T>) -> Box<dyn FooTrait<ValueType = T>> {
        Box::new(Self { a, b, c })
    }
}

struct FooSet<T> {
    v: Vec<Box<dyn FooTrait<ValueType = T>>>,
}

impl<T> FooSet<T>
where
    T: FromStr,
{
    fn new() -> Self {
        Self { v: vec![] }
    }

    fn add(&mut self, v: Box<dyn FooTrait<ValueType = T>>) {
        self.v.push(v);
    }
}

fn main() {
    let a = String::from("hello world");
    let b = 5;
    let c = Some(String::from("optional string"));

    let foo_str = Foo::new(a, b, c);
    println!("foo.a = {}", foo_str.geta());
    println!("foo.b = {}", foo_str.getb());
    println!("foo.c = {:?}", foo_str.getc());

    let d = String::from("hello world");
    let e = 5;
    let f = Some(1024i32);
    let foo_num = Foo::new(d, e, f);
    println!("foo.a = {}", foo_num.geta());
    println!("foo.b = {}", foo_num.getb());
    println!("foo.c = {:?}", foo_num.getc());

    let mut fooset = FooSet::new();
    fooset.add(foo_str);
    fooset.add(foo_num);
}
