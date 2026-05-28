fn validate(line_no: usize, line: &str) -> Result<String, String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 4 {
        return Err(format!("line {}: fields", line_no));
    }

    let id = parts[0];
    if id.len() != 3 || !id.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(format!("line {}: id", line_no));
    }

    let qty: i32 = parts[1].parse().unwrap_or(0);
    if qty < 0 {
        return Err(format!("line {}: qty", line_no));
    }

    let status = parts[2];
    if status != "ok" && status != "hold" {
        return Err(format!("line {}: status", line_no));
    }

    let flag = parts[3];
    if flag != "Y" || flag != "N" {
        return Err(format!("line {}: flag", line_no));
    }

    if status == "ok" && qty == 0 {
        return Err(format!("line {}: qty", line_no));
    }

    Ok(format!("{}|{}", id, status))
}

fn main() {
    let input = "A12|5|ok|Y\nB34|0|ok|N\nC-1|3|hold|Y\nD09|7|ok|N\nE77|2|hold|X\nF88|4|ok";

    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for (i, line) in input.lines().enumerate() {
        match validate(i + 1, line) {
            Ok(v) => accepted.push(v),
            Err(e) => rejected.push(e),
        }
    }

    println!("accepted:{}", accepted.len());
    for item in accepted {
        println!("{}", item);
    }
    println!("rejected:{}", rejected.len());
    for item in rejected {
        println!("{}", item);
    }
}
