use std::cell::RefCell;
use std::rc::Rc;

struct Foo {
    s: String,
}

impl Foo {
    fn new(s: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { s: s.to_string() }))
    }
}

struct Bar {
    vec: Vec<Rc<RefCell<Foo>>>,
}

impl Bar {
    fn new() -> Self {
        Self { vec: vec![] }
    }

    fn add(&mut self, foo: &Rc<RefCell<Foo>>) {
        self.vec.push(Rc::clone(foo));
    }

    fn dot(&mut self) {
        println!("... Dot ...");
        for foo in &self.vec {
            let mut foo = foo.borrow_mut();
            foo.s.push('.');
        }
    }

    fn print(&self) {
        for foo in &self.vec {
            let foo = foo.borrow();
            println!("foo.s: {}", foo.s);
        }
    }
}

fn main() {
    let mut bar = Bar::new();
    let foo_1 = Foo::new("foo_1");
    let foo_2 = Foo::new("foo_2");
    println!(
        "at init..: foo_1 refcount: {}, foo_2 refcount: {}",
        Rc::strong_count(&foo_1),
        Rc::strong_count(&foo_2)
    );
    bar.add(&foo_1);
    bar.add(&foo_2);
    println!(
        "after add: foo_1 refcount: {}, foo_2 refcount: {}",
        Rc::strong_count(&foo_1),
        Rc::strong_count(&foo_2)
    );
    bar.print();
    bar.dot();
    bar.print();
    bar.dot();
    bar.print();
}
