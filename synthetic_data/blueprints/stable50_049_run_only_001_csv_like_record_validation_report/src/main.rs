fn parse_bool(s: &str) -> Option<bool> {
    match s {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn validate_line(line_no: usize, line: &str) -> Result<(), String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 3 {
        return Err(format!("invalid line {}: field count", line_no));
    }

    let name = parts[0].trim();
    if name.is_empty() {
        return Err(format!("invalid line {}: empty name", line_no));
    }

    let age: u8 = parts[1]
        .trim()
        .parse()
        .map_err(|_| format!("invalid line {}: bad age", line_no))?;
    if age > 120 {
        return Err(format!("invalid line {}: bad age", line_no));
    }

    parse_bool(parts[2].trim()).ok_or_else(|| format!("invalid line {}: bad active", line_no))?;
    Ok(())
}

fn main() {
    let input = "alice|30|true\nbob|22\n |44|false\ncarol|200|true\ndave|19|yes\neve|x|false\nfrank|0|false";

    let mut valid = 0usize;
    let mut errors = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        match validate_line(idx + 1, line) {
            Ok(()) => valid += 1,
            Err(e) => errors.push(e),
        }
    }

    println!("valid: {}", valid);
    for e in errors {
        println!("{}", e);
    }
}
