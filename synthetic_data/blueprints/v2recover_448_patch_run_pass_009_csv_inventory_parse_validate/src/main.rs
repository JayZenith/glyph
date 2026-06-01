fn parse_line(line: &str) -> Result<(String, u32), String> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 2 {
        return Err("expected 2 fields".to_string());
    }

    let name = parts[0].trim();
    if name.is_empty() {
        return Err("invalid name".to_string());
    }

    let qty = parts[1].trim().parse::<u32>().map_err(|_| "invalid quantity".to_string())?;
    Ok((name.to_string(), qty))
}

fn main() {
    let input = "apple,3\nbanana,x\n,7\nmelon,4,extra\npear,10";

    for (i, line) in input.lines().enumerate() {
        match parse_line(line) {
            Ok((name, qty)) => println!("ok {} {}", name, qty),
            Err(msg) => println!("err line{} {}", i + 1, msg),
        }
    }
}
