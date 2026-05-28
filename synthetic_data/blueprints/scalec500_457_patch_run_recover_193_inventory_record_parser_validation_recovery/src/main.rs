const INPUT: &str = "id=ABC;qty=2;price=10.50;tags=red,blue\nid=bad1;qty=3;price=4.00;tags=green\nid=XYZ;qty=1;price=3.25;tags=small\nid=LMN;qty=0;price=7.00;tags=ok\nid=QRS;qty=5;price=2.5;tags=ok\nid=TUV;qty=4;price=1.20;tags=a,,b\nid=DUPE;qty=2;price=5.00;tags=x;extra=1\n\n";

fn parse_record(line: &str) -> Option<String> {
    let mut id = None;
    let mut qty = None;
    let mut price = None;
    let mut tags = None;

    for part in line.split(';') {
        let (k, v) = part.split_once('=')?;
        match k {
            "id" => id = Some(v.to_string()),
            "qty" => qty = v.parse::<u32>().ok(),
            "price" => price = v.parse::<f64>().ok(),
            "tags" => tags = Some(v.to_string()),
            _ => {}
        }
    }

    let id = id?;
    if !id.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    if qty? == 0 {
        return None;
    }
    if price? <= 0.0 {
        return None;
    }
    let tags = tags?;
    if tags.is_empty() {
        return None;
    }
    Some(id)
}

fn main() {
    let mut valid = Vec::new();
    let mut invalid = 0;

    for line in INPUT.lines() {
        if line.trim().is_empty() {
            continue;
        }
        match parse_record(line) {
            Some(id) => valid.push(id),
            None => invalid += 1,
        }
    }

    println!("valid: {}", valid.join(","));
    println!("invalid: {}", invalid);
}
