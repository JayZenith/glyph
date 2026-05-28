struct Record {
    name: String,
    age: u32,
    active: bool,
}

fn parse_line(line: &str) -> Result<Record, String> {
    let mut parts = line.split(',');
    let name = parts.next().ok_or("bad name")?;
    let age = parts.next().ok_or("bad age")?;
    let active = parts.next().ok_or("bad active")?;

    if parts.next().is_some() {
        return Err("extra fields".to_string());
    }
    if name.is_empty() {
        return Err("bad name".to_string());
    }

    let age: u32 = age.parse().map_err(|_| "bad age".to_string())?;
    if age > 120 {
        return Err("bad age".to_string());
    }

    let active = match active {
        "true" => true,
        "false" => false,
        _ => return Err("bad active".to_string()),
    };

    Ok(Record {
        name: name.to_string(),
        age,
        active,
    })
}

fn main() {
    let input = "alice,30,true\nbob,130,false\ncarol,25,yes\n,44,true\nerin,0,false";

    let mut valid = 0;
    let mut invalid = 0;
    let mut lines = Vec::new();

    for raw in input.lines() {
        let first = raw.split(',').next().unwrap_or("");
        match parse_line(raw) {
            Ok(rec) => {
                valid += 1;
                let _ = rec.age;
                let _ = rec.active;
                lines.push(format!("{}: ok", rec.name));
            }
            Err(reason) => {
                invalid += 1;
                lines.push(format!("{}: {}", first, reason));
            }
        }
    }

    println!("valid: {}", valid);
    println!("invalid: {}", invalid);
    for line in lines {
        println!("{}", line);
    }
}
