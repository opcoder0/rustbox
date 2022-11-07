pub enum States {
    Start,
    Flag,
    Value,
    BooleanFlag,
    Error,
}

fn main() {
    let bflags: Vec<String> = vec!["-f".to_string(), "-i".to_string()];
    // let oflags: Vec<String> = vec!["-n".to_string(), "-r".to_string()];
    let input: Vec<String> = vec![
        "-n".to_string(),
        "3".to_string(),
        "-f".to_string(),
        "-r".to_string(),
        "5".to_string(),
    ];

    let mut state = States::Start;
    let mut f_name = String::new();
    let mut b_name = String::new();
    for i in input.iter() {
        if i.starts_with("-") {
            // flag
            if is_bool_flag(&i, &bflags) {
                state = States::BooleanFlag;
                b_name.push_str(&i.clone());
            } else {
                state = States::Flag;
                f_name.push_str(&i.clone());
            }
        } else {
            // value
            match state {
                States::Start => {
                    println!("expecting flag; found value");
                    state = States::Error;
                    break;
                }
                States::Flag => {
                    // consume the value;
                    println!("flag {} consumes value: {}", f_name, i);
                    f_name.clear();
                }
                States::BooleanFlag => {
                    if i == "true" || i == "false" {
                        // setting boolean value to flag
                        println!("boolean flag {} consumes value {}", f_name, i);
                        f_name.clear();
                    } else {
                        println!("invalid boolean value; expecting true or false or nothing");
                        state = States::Error;
                        break;
                    }
                }
                States::Value => {
                    println!("flag must be followed by a single value. Enclose it in quotes");
                    state = States::Error;
                    break;
                }
                States::Error => {
                    println!("error state");
                    break;
                }
            }
        }
    }
    match state {
        States::Start => {
            println!("if there were mandatory flags; prints error here - missing mandatory flags");
        }
        States::Flag => {
            println!("missing value for flag");
        }
        States::BooleanFlag => {
            println!("ok");
        }
        States::Value => {
            println!("ok");
        }
        States::Error => {
            println!("error");
        }
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
