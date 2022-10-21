use std::any::Any;
use std::str::FromStr;

struct Flag<T> {
    pub option: String,
    pub description: String,
    value: Option<T>,
    pub default_value: Option<T>,
    pub mandatory: bool,
}

fn main() {
    let flags: Vec<Box<dyn Any>>;

    let mut iflag = Flag {
        option: String::from("ignore"),
        description: String::from("ignore case"),
        value: None,
        default_value: None,
        mandatory: false,
    };

    iflag.value = Some(5i32);
}
