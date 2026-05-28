use std::collections::HashSet;

const INPUT: &str = "id=U1;name=Ana;age=30\nid=U2;name=Bob;age=xx\nid=U1;name=Eve;age=22\nid=U3;name=;age=19\nid=U4;name=Dee;age=40;note=vip";

fn validate_line(line: &str, seen: &mut HashSet<String>) -> Option<&'static str> {
    let mut id = "";
    let mut name = "";
    let mut age = "";

    for field in line.split(';') {
        let mut parts = field.split('=');
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");
        match key {
            "id" => id = value,
            "name" => name = value,
            "age" => age = value,
            _ => {}
        }
    }

    if id.is_empty() {
        return Some("missing id");
    }
    if !seen.insert(id.to_string()) {
        return Some("duplicate id");
    }
    if name.is_empty() {
        return Some("empty name");
    }
    if age.parse::<u8>().is_err() {
        return Some("bad age");
    }
    None
}

fn main() {
    let mut seen = HashSet::new();
    let mut bad = Vec::new();

    for (idx, line) in INPUT.lines().enumerate() {
        if let Some(reason) = validate_line(line, &mut seen) {
            bad.push(format!("{}: {}", idx + 1, reason));
        }
    }

    println!("invalid={}", bad.len());
    for item in bad {
        println!("{}", item);
    }
}
