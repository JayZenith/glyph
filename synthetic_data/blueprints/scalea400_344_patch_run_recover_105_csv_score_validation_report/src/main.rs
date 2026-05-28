const INPUT: &str = "Alice,5,true\nBob,12,false\n,3,true\nCara,7\nDora,8,yes\nFrank,2,false";

fn parse_line(line: &str) -> Result<Option<(String, i32)>, String> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("wrong field count".to_string());
    }

    let name = parts[0].trim();
    if name.is_empty() {
        return Err("empty name".to_string());
    }

    let score: i32 = parts[1].trim().parse().map_err(|_| "bad score".to_string())?;
    if !(0..=10).contains(&score) {
        return Err("bad score".to_string());
    }

    let active = matches!(parts[2].trim(), "true" | "yes");
    Ok(Some((name.to_string(), score * if active { 1 } else { 0 })))
}

fn main() {
    let mut valid = 0;
    let mut invalid = 0;
    let mut rows = Vec::new();
    let mut errors = Vec::new();

    for (idx, line) in INPUT.lines().enumerate() {
        match parse_line(line) {
            Ok(Some((name, score))) => {
                valid += 1;
                rows.push(format!("{}={}", name, score));
            }
            Ok(None) => {}
            Err(msg) => {
                invalid += 1;
                errors.push(format!("ERR line {}: {}", idx + 1, msg));
            }
        }
    }

    println!("valid: {}", valid);
    println!("invalid: {}", invalid);
    for row in rows {
        println!("{}", row);
    }
    for err in errors {
        println!("{}", err);
    }
}
