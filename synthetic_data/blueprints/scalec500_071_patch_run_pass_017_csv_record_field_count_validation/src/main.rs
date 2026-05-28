fn parse_record(line: &str) -> Option<(String, u32, bool)> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return None;
    }

    let name = parts[0];
    if name.is_empty() {
        return None;
    }

    let age: u32 = parts[1].parse().ok()?;
    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return None,
    };

    Some((name.to_string(), age, active))
}

fn main() {
    let input = [
        "Alice,30,true",
        "Bob,notnum,false",
        "Cara,25,maybe",
        "Dan,40",
        "Eve,22,true,extra",
    ];

    for line in input {
        match parse_record(line) {
            Some((name, age, active)) => println!("ok: {} {} {}", name, age, active),
            None => println!("invalid: {}", line),
        }
    }
}
