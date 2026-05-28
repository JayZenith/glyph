fn parse_line(line_no: usize, line: &str) -> Result<(), String> {
    let mut id = "";
    let mut name = "";
    let mut score = "";

    for part in line.split(';') {
        let Some((key, value)) = part.split_once('=') else {
            return Err(format!("line {}: malformed field", line_no));
        };
        match key {
            "id" => id = value,
            "name" => name = value,
            "score" => score = value,
            _ => {}
        }
    }

    if id.is_empty() {
        return Err(format!("line {}: missing id", line_no));
    }
    if name.is_empty() {
        return Err(format!("line {}: missing name", line_no));
    }
    if score.is_empty() {
        return Err(format!("line {}: missing score", line_no));
    }

    if id.parse::<u32>().is_err() {
        return Err(format!("line {}: invalid id", line_no));
    }
    if score.parse::<u32>().is_err() {
        return Err(format!("line {}: invalid score", line_no));
    }

    Ok(())
}

fn main() {
    let input = "id=10;name=Alice;score=42\nid=xy;name=Bob;score=17\nid=12;name=Cara\nid=13;name=Dan;score=high\nid=14;name=;score=8\nid=15;name=Eve;score=9";

    let mut valid = 0;
    let mut invalid = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        match parse_line(idx + 1, line) {
            Ok(()) => valid += 1,
            Err(msg) => invalid.push(msg),
        }
    }

    println!("valid: {}", valid);
    for msg in invalid {
        println!("invalid: {}", msg);
    }
}
