struct Foo {
    s: String,
}

impl Foo {
    fn new(s: &str) -> Self {
        Self { s: s.to_string() }
    }
}

struct Bar {
    vec: Vec<Foo>,
}

impl Bar {
    fn new() -> Box<Self> {
        Box::new(Self { vec: vec![] })
    }

    fn add(&mut self, foo: Foo) {
        self.vec.push(foo);
    }

    fn print(&self) {
        println!("*** Bar::vec ***");
        for foo in &self.vec {
            println!("foo.s: {}", foo.s);
        }
    }
}

fn main() {
    let mut bar = Bar::new();
    let foo_1 = Foo::new("foo_1");
    let foo_2 = Foo::new("foo_2");
    bar.add(foo_1);
    bar.add(foo_2);
    bar.print();
}
