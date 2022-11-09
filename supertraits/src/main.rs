pub trait Foo {
    fn say_foo(&self);
}

trait Bar: Foo {
    fn say_bar(&self);
}

struct FooBar {
    s: String,
}

impl FooBar {
    fn new() -> Self {
        Self {
            s: "FooBar::new() returns FooBar as Self:".to_string(),
        }
    }

    fn new_1() -> Box<dyn Bar> {
        let s = Self {
            s: "FooBar::new_1() returns FooBar as dyn Bar:".to_string(),
        };
        return Box::new(s);
    }

    fn new_2() -> Box<dyn Foo> {
        let s = Self {
            s: "FooBar::new_2() returns FooBar as dyn Foo:".to_string(),
        };
        return Box::new(s);
    }
}

impl Foo for FooBar {
    fn say_foo(&self) {
        println!("Saying foo from say_foo: {}", self.s);
    }
}

impl Bar for FooBar {
    fn say_bar(&self) {
        println!("Saying bar from say_bar: {}", self.s);
    }
}

fn main() {
    println!("*** new() ***");
    let foo_1 = FooBar::new();
    foo_1.say_foo();
    foo_1.say_bar();
    println!("*** new_1() ***");
    let bar_1 = FooBar::new_1(); // can call say_foo and say_bar via dyn Bar
    bar_1.say_bar();
    bar_1.say_foo();
    println!("*** new_2() ***");
    let foo_2 = FooBar::new_2();
    foo_2.say_foo();
    // foo_2.say_bar(); // cannot call say_bar via dyn Foo

    let mut v: Vec<Box<dyn Foo>> = Vec::new();
    v.push(Box::new(foo_1));
    v.push(Box::new(bar_1));
}
