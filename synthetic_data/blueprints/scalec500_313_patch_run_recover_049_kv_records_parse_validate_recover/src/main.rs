const INPUT: &str = "name=Alice;age=30;role=admin\nname=Bob;age=22;age=23\nage=19;role=user\nname=Eve;age=130;role=user\nname=Mallory;age=28;city=Rome\nname=Trent;age=41;role=user";

fn validate_line(line: &str) -> Result<(), String> {
    let mut name = None;
    let mut age = None;

    for part in line.split(';') {
        let (key, value) = part.split_once('=').ok_or_else(|| "bad field".to_string())?;
        match key {
            "name" => name = Some(value),
            "age" => {
                let n: u32 = value.parse().map_err(|_| "bad age".to_string())?;
                age = Some(n);
            }
            "role" => {}
            _ => {}
        }
    }

    if name.is_none() {
        return Err("missing name".to_string());
    }
    match age {
        Some(n) if n <= 120 => {}
        Some(_) => return Err("age out of range".to_string()),
        None => return Err("missing age".to_string()),
    }
    Ok(())
}

fn main() {
    let mut out = Vec::new();
    let mut valid = 0;

    for (i, line) in INPUT.lines().enumerate() {
        match validate_line(line) {
            Ok(()) => valid += 1,
            Err(msg) => out.push(format!("invalid: line {} {}", i + 1, msg)),
        }
    }

    println!("valid: {}", valid);
    for line in out {
        println!("{}", line);
    }
}
