fn main() {
    let s = shortname(1);
    let l = longname(3);
    let v = l.unwrap_or_else(|| {
        let v = s.unwrap_or_else(|| {
            panic!("shortname and longname are both missing");
        });
        v.to_string()
    });
    println!("v: {}", v);
}

fn shortname(i: i32) -> Option<char> {
    if i % 2 == 0 {
        Some('r')
    } else {
        None
    }
}

fn longname(i: i32) -> Option<String> {
    if i % 2 == 0 {
        Some("retry".to_string())
    } else {
        None
    }
}
