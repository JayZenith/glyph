use std::collections::HashMap;

const INPUT: &str = "id=12;qty=4;active=yes
id=77;qty=0;active=yes
id=ab;qty=5;active=no
id=90;qty=3;active=no
id=55;active=yes
id=22;qty=7;active=maybe
id=10;qty=2;qty=9;active=yes";

fn parse_line(line: &str) -> Option<(String, i32, String)> {
    let mut map = HashMap::new();
    for part in line.split(';') {
        let (k, v) = part.split_once('=')?;
        map.insert(k, v);
    }

    let id = map.get("id")?;
    let qty = map.get("qty")?.parse::<i32>().ok()?;
    let active = map.get("active")?;

    if qty < 0 {
        return None;
    }

    Some((id.to_string(), qty, active.to_string()))
}

fn main() {
    let mut valid = 0;
    let mut invalid = 0;

    for line in INPUT.lines() {
        match parse_line(line) {
            Some((id, qty, active)) => {
                valid += 1;
                println!("OK {} {} {}", id, qty, active);
            }
            None => invalid += 1,
        }
    }

    println!("valid={} invalid={}", valid, invalid);
}
