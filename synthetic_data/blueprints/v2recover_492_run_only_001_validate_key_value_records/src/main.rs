fn validate(line: &str) -> &'static str {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 3 {
        return "field_count";
    }

    let mut name = None;
    let mut age = None;
    let mut active = None;

    for part in parts {
        let mut kv = part.splitn(2, '=');
        let key = kv.next().unwrap_or("");
        let value = kv.next().unwrap_or("");
        match key {
            "name" => name = Some(value),
            "age" => age = Some(value),
            "active" => active = Some(value),
            _ => return "unknown_key",
        }
    }

    if name.unwrap_or("").is_empty() {
        return "missing_name";
    }

    match age.unwrap_or("").parse::<u32>() {
        Ok(n) if n <= 120 => {}
        _ => return "bad_age",
    }

    match active.unwrap_or("") {
        "true" | "false" => {}
        _ => return "bad_active",
    }

    "ok"
}

fn main() {
    let input = [
        "name=Ana|age=30|active=true",
        "name=Bob|age=200|active=false",
        "name=|age=44|active=true",
        "name=Eve|age=27|active=yes",
        "active=false|age=19|name=Kai",
        "name=Zoe|age=18",
    ];

    let mut valid = 0;
    let mut invalid = 0;
    let mut lines = Vec::new();

    for (idx, line) in input.iter().enumerate() {
        let status = validate(line);
        if status == "ok" {
            valid += 1;
            lines.push(format!("{}:ok", idx + 1));
        } else {
            invalid += 1;
            lines.push(format!("{}:invalid:{}", idx + 1, status));
        }
    }

    println!("valid={} invalid={}", valid, invalid);
    print!("{}", lines.join("\n"));
}
