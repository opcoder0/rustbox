use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct FlagParseError;

impl fmt::Display for FlagParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "flag value cannot be parsed to the specified type".fmt(f)
    }
}

struct GenVal<T> {
    gen_val: T,
}

impl<T> GenVal<T> {
    fn new(gen_val: T) -> Self {
        Self { gen_val }
    }

    fn value(&self) -> &T {
        &self.gen_val
    }

    fn parse<F: FromStr>(&self, s: &str) -> Result<F, F::Err> {
        FromStr::from_str(s)
    }
}

fn main() {
    let g = GenVal::new(5i32);
    // let gval = g.value();
    let p = g.parse::<i32>("5");
    match p {
        Ok(result) => {
            println!("result: {}", result);
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}
