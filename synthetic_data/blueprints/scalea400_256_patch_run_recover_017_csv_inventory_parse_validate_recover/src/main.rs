const INPUT: &str = "A12,Widget,10\nB03,Gadget,0\nC77,,5\nX9,Thing,3\nD44,Part,abc\nE55, Bolt ,7";

fn parse_line(line: &str) -> Result<(&str, &str, i32), &'static str> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 3 {
        return Err("bad field count");
    }

    let id = parts[0];
    let name = parts[1];
    let qty: i32 = parts[2].parse().map_err(|_| "bad qty")?;

    if id.len() < 2 || !id.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err("bad id");
    }
    if name.is_empty() {
        return Err("missing name");
    }
    if qty < 0 {
        return Err("qty not positive");
    }

    Ok((id, name, qty))
}

fn main() {
    let mut valid = 0;
    let mut invalid = 0;
    let mut total_qty = 0;
    let mut errors = Vec::new();

    for (idx, line) in INPUT.lines().enumerate() {
        match parse_line(line) {
            Ok((_id, _name, qty)) => {
                valid += 1;
                total_qty += qty;
            }
            Err(msg) => {
                invalid += 1;
                errors.push(format!("ERR line {}: {}", idx + 1, msg));
            }
        }
    }

    println!("valid={} invalid={} total_qty={}", valid, invalid, total_qty);
    for err in errors {
        println!("{}", err);
    }
}
