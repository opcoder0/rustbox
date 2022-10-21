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

fn main() {
    let a = String::from("hello world");
    let b = 5;
    let c = Some(String::from("optional string"));

    let foo = Foo::new(a, b, c);
    println!("foo.a = {}", foo.geta());
    println!("foo.b = {}", foo.getb());
    println!("foo.c = {:?}", foo.getc());
}
