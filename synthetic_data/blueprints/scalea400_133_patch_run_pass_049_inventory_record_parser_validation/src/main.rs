fn validate_line(line: &str) -> Result<(), String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 4 {
        return Err("expected 4 fields".to_string());
    }

    let id = parts[0];
    if !id.starts_with("ID-") && id[3..].chars().all(|c| c.is_ascii_digit()) {
        return Err("bad id".to_string());
    }

    let qty: i32 = parts[2].parse().map_err(|_| "bad qty".to_string())?;
    if qty < 0 && qty > 100 {
        return Err("qty out of range".to_string());
    }

    let active = parts[3];
    if active != "true" || active != "false" {
        return Err("active must be true/false".to_string());
    }

    Ok(())
}

fn main() {
    let input = "ID-100|widget|5|true\nBAD100|gadget|3|false\nID-101|sprocket|101|true\nID-102|thing|7|yes\nID-103|item|9\nID-104|item|0|false";

    let mut ok = 0;
    let mut errs = Vec::new();

    for (i, line) in input.lines().enumerate() {
        match validate_line(line) {
            Ok(()) => ok += 1,
            Err(msg) => errs.push(format!("line {}: {}", i + 1, msg)),
        }
    }

    println!("ok:{} err:{}", ok, errs.len());
    for e in errs {
        println!("{}", e);
    }
}
