use std::collections::HashMap;
use std::env;
use std::str::FromStr;

pub struct FlagValue<'a, T> {
    v: Option<T>,
}

impl FromStr for FlagValue<T> {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s.parse::<bool>();
    }
}

pub struct Flag<'a, T> {
    shortname: Option<char>,
    longname: Option<&'a str>,
    description: &'a str,
    default_value: Option<T>,
    mandatory: bool,
    value: Option<T>,
}

impl<'a, T> Flag<'a, T> {
    pub fn new(
        shortname: Option<char>,
        longname: Option<&'a str>,
        description: &'a str,
        default_value: Option<T>,
        mandatory: bool,
    ) -> Self {
        Self {
            shortname,
            longname,
            description,
            default_value,
            mandatory,
            value: None,
        }
    }
}

pub struct FlagSet<'a, T> {
    flag_pair: HashMap<String, String>,
    flag_map: HashMap<String, Flag<'a, T>>,
}

impl<'a, T> FlagSet<'a, T>
where
    T: Clone,
{
    pub fn new() -> Self {
        let flag_map = HashMap::new();
        let flag_pair = HashMap::new();
        Self {
            flag_pair,
            flag_map,
        }
    }

    pub fn add(&mut self, f: Flag<'a, T>) {
        if f.shortname == None && f.longname == None {
            panic!("required: short name or long name");
        }
        if let (Some(shortname), Some(longname)) = (&f.shortname, &f.longname) {
            self.flag_pair
                .insert(longname.to_string().clone(), shortname.to_string().clone());
            self.flag_pair
                .insert(shortname.to_string().clone(), longname.to_string().clone());
            self.flag_map.insert(shortname.to_string().clone(), f);
            return;
        }
        let has_short_flag = f.shortname.is_some();
        let has_long_flag = f.longname.is_some();
        if has_short_flag {
            let shortname = f.shortname.expect("missing short flag name");
            self.flag_map.insert(shortname.to_string().clone(), f);
            return;
        }
        if has_long_flag {
            let longname = f.longname.expect("missing long flag name");
            self.flag_map.insert(longname.to_string().clone(), f);
        }
    }

    pub fn value_of(&self, argname: &'a str) -> Result<&Option<FlagValue>, String> {
        if !self.is_valid_flag(argname) {
            return Err("invalid flag name".to_string());
        }
        let f = self.flag_map.get(argname);
        match f {
            Some(flag) => {
                return Ok(&flag.value);
            }
            None => {
                return Err("value not set".to_string());
            }
        }
    }

    pub fn parse(&mut self, args: &mut env::Args) -> Result<(), String> {
        args.next(); // skip the program name.
        loop {
            if let Some(arg) = args.next() {
                if arg.starts_with("-") {
                    let argname = arg.trim_start_matches("-");
                    if !self.is_valid_flag(argname) {
                        return Err(format!("invalid flag name {}", arg));
                    }
                    let flag = self.flag_map.get_mut(argname);
                    match flag {
                        Some(f) => {
                            let argval = args.next().unwrap_or("".to_string());
                            match Self::set_flag_value(argval, f) {
                                Ok(_) => {}
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        }
                        None => {
                            return Err(format!("flag not found"));
                        }
                    }
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn set_flag_value(argval: String, flag: &mut Flag<'a, T>) -> Result<(), String> {
        match argval.len() {
            0 => {
                return Err("missing expected value".to_string());
            }
            _ => {
                if argval.starts_with("-") {
                    // looks like the next flag.
                    // use the default value if available.
                    if flag.default_value.is_some() {
                        flag.value = flag.default_value.clone();
                        return Ok(());
                    } else {
                        return Err("missing expected value".to_string());
                    }
                }
                flag.value = Some(argval);
                match flag.value_type {
                    FlagValue::I32(_) => match argval.parse::<i32>() {
                        Ok(ival) => {
                            flag.value = Some(FlagValue::I32(ival));
                            return Ok(());
                        }
                        Err(_) => {
                            return Err(format!(
                                "invalid value {} expecting a signed integer type",
                                argval
                            ));
                        }
                    },
                    FlagValue::U32(_) => match argval.parse::<u32>() {
                        Ok(uval) => {
                            flag.value = Some(FlagValue::U32(uval));
                            return Ok(());
                        }
                        Err(_) => {
                            return Err(format!(
                                "invalid value {} expecting an unsigned integer type",
                                argval
                            ));
                        }
                    },
                    FlagValue::Text(_) => {
                        flag.value = Some(FlagValue::Text(argval.to_owned()));
                        return Ok(());
                    }
                    FlagValue::Choice(_) => {
                        flag.value = Some(FlagValue::Choice(true));
                        return Ok(());
                    }
                }
            }
        }
    }

    fn is_valid_flag(&self, k: &str) -> bool {
        self.flag_map.contains_key(k)
    }
}

fn main() {
    let ignore_backup = Flag::new(
        Some('b'),
        Some("ignore-backup"),
        "ignore backup",
        Some(true),
        false,
    );
    let retry_flag = Flag::new(
        Some('r'),
        Some("retry"),
        "number of retries",
        Some(3),
        false,
    );
}
