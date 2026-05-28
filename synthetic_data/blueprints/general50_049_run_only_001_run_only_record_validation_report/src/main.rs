fn validate_line(line: &str, seen: &mut Vec<String>) -> String {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 2 {
        return "wrong field count".to_string();
    }

    let key = parts[0];
    let value = parts[1];

    if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()) {
        return "key must be lowercase alphabetic".to_string();
    }
    if seen.iter().any(|k| k == key) {
        return "duplicate key".to_string();
    }
    if value.is_empty() {
        return "missing value".to_string();
    }

    match value.parse::<i32>() {
        Ok(n) if n >= 0 => {
            seen.push(key.to_string());
            format!("VALID {}={}", key, n)
        }
        _ => "bad integer".to_string(),
    }
}

fn main() {
    let input = "alpha|10\nalpha|11\nbeta|\ngamma|x\nmi-x|3\nOMEGA|7\nzeta|9|extra\ndelta|0";
    let mut seen = Vec::new();
    let mut valid = 0;
    let mut invalid = 0;
    let mut sum = 0i32;
    let mut out = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let result = validate_line(line, &mut seen);
        if let Some(rest) = result.strip_prefix("VALID ") {
            valid += 1;
            if let Some((_, n)) = rest.split_once('=') {
                sum += n.parse::<i32>().unwrap();
            }
            out.push(result);
        } else {
            invalid += 1;
            out.push(format!("INVALID line {}: {}", idx + 1, result));
        }
    }

    out.push(format!("SUMMARY valid={} invalid={} sum={}", valid, invalid, sum));
    print!("{}", out.join("\n"));
}
