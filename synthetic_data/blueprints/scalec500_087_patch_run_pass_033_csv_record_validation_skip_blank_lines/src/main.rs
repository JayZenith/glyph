const INPUT: &str = "1,Alice,admin\n\n2,,user\n3,Carol,guest\n4,Dave,owner\n";

fn parse_line(line: &str) -> Result<(u32, &str, &str), &'static str> {
    let mut parts = line.split(',');
    let id = parts.next().ok_or("missing id")?;
    let name = parts.next().ok_or("missing name")?;
    let role = parts.next().ok_or("missing role")?;

    if parts.next().is_some() {
        return Err("too many fields");
    }

    let id_num: u32 = id.parse().map_err(|_| "bad id")?;

    if name.is_empty() {
        return Err("empty name");
    }

    match role {
        "admin" | "user" | "guest" => Ok((id_num, name, role)),
        _ => Err("bad role"),
    }
}

fn main() {
    let mut valid = 0;
    let mut invalid = 0;

    for line in INPUT.lines() {
        match parse_line(line) {
            Ok((id, name, _role)) => {
                println!("OK {} {}", id, name);
                valid += 1;
            }
            Err(_) => invalid += 1,
        }
    }

    println!("valid={} invalid={}", valid, invalid);
}
