use std::default::Default;
use std::str::FromStr;

trait ValueType {
    fn value_of(&mut self, s: &str);
}

fn value_of<F>(s: &str) -> Result<F, <F as FromStr>::Err>
where
    F: FromStr,
{
    s.parse::<F>()
}

trait CommandlineArgument {
    fn shortname(&self) -> Option<&char>;
    fn longname(&self) -> Option<&String>;
    fn description(&self) -> &String;
    fn mandatory(&self) -> bool;
    fn get_value(&self) -> Option<&dyn ValueType>;
    fn set_value(&mut self, v: Box<dyn ValueType>);
}

struct Flag {
    shortname: Option<char>,
    longname: Option<String>,
    description: String,
    mandatory: bool,
    value: Option<Box<dyn ValueType>>,
}

impl Flag {
    fn new(
        shortname: Option<char>,
        longname: Option<String>,
        description: String,
        mandatory: bool,
    ) -> Self {
        Self {
            shortname,
            longname,
            description,
            mandatory,
            value: None,
        }
    }
}

impl CommandlineArgument for Flag {
    fn shortname(&self) -> Option<&char> {
        self.shortname.as_ref()
    }

    fn longname(&self) -> Option<&String> {
        self.longname.as_ref()
    }

    fn description(&self) -> &String {
        &self.description
    }

    fn mandatory(&self) -> bool {
        self.mandatory
    }

    fn get_value(&self) -> Option<&dyn ValueType> {
        let v = self.value.as_ref();
        match v {
            Some(v) => Some(v.as_ref()),
            None => None,
        }
    }

    fn set_value(&mut self, v: Box<dyn ValueType>) {
        self.value = Some(v);
    }
}

struct FlagSet {
    mix: Vec<Box<dyn CommandlineArgument>>,
}

impl FlagSet {
    fn new() -> Self {
        Self { mix: vec![] }
    }
}

impl<T> ValueType for T
where
    T: FromStr + Default,
{
    fn value_of(&mut self, s: &str) {
        let r = Self::from_str(s);
        if r.is_ok() {
            *self = r.unwrap_or_default();
        }
    }
}

fn main() {
    let mut flagset = FlagSet::new();
}
