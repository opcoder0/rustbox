pub trait Flag {
    type FlagValue;
    fn define(
        shortname: Option<char>,
        longname: Option<String>,
        description: String,
        mandatory: bool,
        default: Option<Self::FlagValue>,
    ) -> Box<Self>;
}

struct FlagImpl<T> {
    shortname: Option<char>,
    longname: Option<String>,
    description: String,
    mandatory: bool,
    default: Option<T>,
    value: Option<T>,
}

impl<T> Flag for FlagImpl<T> {
    type FlagValue = T;

    fn define(
        shortname: Option<char>,
        longname: Option<String>,
        description: String,
        mandatory: bool,
        default: Option<Self::FlagValue>,
    ) -> Box<Self> {
        Box::new(Self {
            shortname,
            longname,
            description,
            mandatory,
            default,
            value: None,
        })
    }
}

fn main() {
    let i_flag = Flag::define(
        Some('i'),
        Some("iter".to_string()),
        "iterate specified times".to_string(),
        false,
        Some(3),
    );
}
