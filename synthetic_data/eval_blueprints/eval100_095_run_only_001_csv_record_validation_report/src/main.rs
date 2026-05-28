fn parse_bool(s: &str) -> Option<bool> {
    match s {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn main() {
    let input = "1,Alice,30,true\n2,Bob,xx,false\nX,Carol,22,true\n4,Dan,41,maybe\n5,Eve,27,false\n6,Frank,18,true,extra";

    let mut valid = 0usize;
    let mut invalid = 0usize;
    let mut errors = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let line_no = idx + 1;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 4 {
            invalid += 1;
            errors.push(format!("line {}: wrong field count", line_no));
            continue;
        }

        if parts[0].parse::<u32>().is_err() {
            invalid += 1;
            errors.push(format!("line {}: bad id", line_no));
            continue;
        }
        if parts[1].is_empty() {
            invalid += 1;
            errors.push(format!("line {}: bad name", line_no));
            continue;
        }
        if parts[2].parse::<u8>().is_err() {
            invalid += 1;
            errors.push(format!("line {}: bad age", line_no));
            continue;
        }
        if parse_bool(parts[3]).is_none() {
            invalid += 1;
            errors.push(format!("line {}: bad active", line_no));
            continue;
        }

        valid += 1;
    }

    println!("valid: {}", valid);
    println!("invalid: {}", invalid);
    println!("errors:");
    for err in errors {
        println!("{}", err);
    }
}
