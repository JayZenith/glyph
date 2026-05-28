fn valid_record(line: &str) -> Option<&str> {
    let mut parts = line.split(',');
    let name = parts.next()?;
    let age: u32 = parts.next()?.parse().ok()?;
    let score: u32 = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }

    if !name.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }

    if age > 18 && score <= 100 {
        Some(name)
    } else {
        None
    }
}

fn main() {
    let input = "Ada,18,100\nBob,17,91\nCy,20,101\nD4n,22,88\nEli,30,0";
    let mut valid = 0;
    let mut invalid = 0;

    for line in input.lines() {
        match valid_record(line) {
            Some(name) => {
                valid += 1;
                println!("ok: {}", name);
            }
            None => invalid += 1,
        }
    }

    println!("valid={} invalid={}", valid, invalid);
}
