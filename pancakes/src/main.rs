use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Flag<'_, T> {
    shortname: Option<char>,
    longname: Option<&'a str>,
    description: &'a str,
    default_value: Option<T>,
    mandatory: bool,
    value: Option<T>,
}

fn main() {
    Flag::<i32>::hello_macro();
}
