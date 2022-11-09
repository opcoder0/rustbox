#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum State {
    Start,
    Flag,
    Value,
    BooleanFlag,
    Error,
}

fn main() {
    let bflags: Vec<String> = vec!["-f".to_string(), "-i".to_string()];
    // let oflags: Vec<String> = vec!["-n".to_string(), "-r".to_string()];
    //
    //    let input: Vec<String> = vec![
    //     "-n".to_string(),
    //     "3".to_string(),
    //     "-f".to_string(),
    //     "-r".to_string(),
    //     "5".to_string(),
    // ];

    // invalid input
    // let input: Vec<String> = vec![
    //     "-f".to_string(),
    //     "-n".to_string(),
    //     "3".to_string(),
    //     "5".to_string(),
    //     "-r".to_string(),
    //     "5".to_string(),
    // ];

    // overriding/repeating inputs
    let input: Vec<String> = vec![
        "-f".to_string(),
        "-n".to_string(),
        "5".to_string(),
        "-r".to_string(),
        "5".to_string(),
        "-n".to_string(),
        "6".to_string(),
        "-f".to_string(),
    ];

    let mut state = State::Start;
    let mut fname = String::new();
    let mut bname = String::new();
    for i in input.iter() {
        if i.starts_with("-") {
            if is_bool_flag(&i, &bflags) {
                match state {
                    State::Start => {
                        bname.push_str(&i.clone());
                        state = State::BooleanFlag;
                    }
                    State::BooleanFlag => {
                        if bname.is_empty() {
                            bname.push_str(&i.clone());
                            state = State::BooleanFlag;
                        } else {
                            // process the previous boolean flag
                            println!("boolean flag: {} => true", bname);
                            bname.clear();
                            bname.push_str(&i.clone());
                            state = State::BooleanFlag;
                        }
                    }
                    State::Value => {
                        bname.clear();
                        bname.push_str(&i.clone());
                        state = State::BooleanFlag;
                    }
                    _ => {
                        println!("invalid state transition to boolean flag");
                        state = State::Error;
                        break;
                    }
                }
            } else {
                match state {
                    State::Start => {
                        fname.push_str(&i.clone());
                        state = State::Flag;
                    }
                    State::BooleanFlag => {
                        println!("boolean flag: {} => true", bname);
                        bname.clear();
                        fname.clear();
                        fname.push_str(&i.clone());
                        state = State::Flag;
                    }
                    State::Value => {
                        fname.clear();
                        fname.push_str(&i.clone());
                        state = State::Flag;
                    }
                    _ => {
                        println!("missing required value: {}", fname);
                        state = State::Error;
                        break;
                    }
                }
            }
        } else {
            match state {
                State::Flag => {
                    println!("{} = {}", fname, i);
                    fname.clear();
                    state = State::Value;
                }
                State::BooleanFlag => {
                    if i == "true" || i == "false" {
                        println!("boolean flag: {} => {}", bname, i);
                        bname.clear();
                        state = State::Value;
                    } else {
                        println!("boolean flag can only have true/false");
                        state = State::Error;
                        break;
                    }
                }
                _ => {
                    println!("a flag must precede the value");
                    state = State::Error;
                    break;
                }
            }
        }
    }
    if state != State::Error {
        println!("ok");
    } else {
        println!("error parsing arguments");
    }
}

fn is_bool_flag(v: &String, bflags: &Vec<String>) -> bool {
    if v.starts_with("-") {
        for flag in bflags {
            if v == flag {
                return true;
            }
        }
    }
    return false;
}
