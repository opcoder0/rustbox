use std::env;

#[derive(Debug)]
pub enum FlagType {
    I8(i8),
    U8(u8),
    Bool(bool),
    Text(String),
}

pub struct Flag<'a> {
    pub shortname: Option<char>,
    pub longname: Option<&'a str>,
    pub description: &'a str,
    pub default_value: Option<&'a FlagType>,
    pub mandatory: bool,
    pub value: &'a mut FlagType,
}

fn main() {
    let mut fval: FlagType = FlagType::U8(0);
    let mut f1val: FlagType = FlagType::U8(0);
    let mut f = Flag {
        shortname: Some('n'),
        longname: None,
        description: "run test specified number of times",
        default_value: None,
        mandatory: true,
        value: &mut fval,
    };
    let mut f1 = Flag {
        shortname: Some('t'),
        longname: Some("timeout"),
        description: "timeout in seconds",
        default_value: Some(&FlagType::U8(5)),
        mandatory: false,
        value: &mut f1val,
    };
    let mut args = env::args();
    args.next(); // skip the first argument (name of the program)
    parse(&mut f, &mut f1, &mut args);

    println!("parse complete");
    println!("f.value: {:?}", f.value);
    println!("f1.value: {:?}", f1.value);
}

fn parse(f: &mut Flag, f1: &mut Flag, args: &mut env::Args) {
    println!("parse:");
    let mut flag_found = false;
    loop {
        let arg = match args.next() {
            Some(arg) => arg,
            None => break,
        };
        if arg.starts_with("-") {
            let argname = arg.trim_start_matches("-");
            match argname.len() {
                1 => {
                    let c = match argname.chars().next() {
                        Some(c) => c,
                        None => {
                            panic!("invalid option format")
                        }
                    };
                    if let Some(c1) = f.shortname {
                        if c1 == c {
                            flag_found = true;
                            match args.next() {
                                Some(val) => {
                                    let ival = val.parse::<i8>().unwrap();
                                    *f.value = FlagType::I8(ival);
                                }
                                None => match f.default_value {
                                    Some(dv) => match dv {
                                        FlagType::I8(ival) => *f.value = FlagType::I8(*ival),
                                        _ => {
                                            todo!()
                                        }
                                    },
                                    None => {
                                        panic!("missing expected value for flag")
                                    }
                                },
                            }
                        }
                    }
                    if flag_found == false {
                        if let Some(c1) = f1.shortname {
                            if c1 == c {
                                flag_found = true;
                                match args.next() {
                                    Some(val) => {
                                        let ival = val.parse::<i8>().unwrap();
                                        *f1.value = FlagType::I8(ival);
                                    }
                                    None => match f1.default_value {
                                        Some(dv) => match dv {
                                            FlagType::I8(ival) => *f1.value = FlagType::I8(*ival),
                                            _ => {
                                                todo!()
                                            }
                                        },
                                        None => {
                                            panic!("missing expected value for flag")
                                        }
                                    },
                                }
                            }
                        }
                    }
                    if flag_found == false {
                        panic!("invalid command line option");
                    }
                }
                _ => {
                    todo!()
                }
            }
        }
    }
}
