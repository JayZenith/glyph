fn parse_line(line: &str) -> Option<u32> {
    let mut parts = line.split('|');
    let kind = parts.next()?;
    let id = parts.next()?;
    let status = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    if kind != "item" {
        return None;
    }
    if status != "ok" {
        return None;
    }
    if id.is_empty() || !id.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    id.parse().ok()
}

fn main() {
    let input = "item|42|ok\nitem|x|ok\nuser|9|ok\nitem|7|ok\nitem|8|bad\nitem||ok\nitem|15|ok";
    let mut ids = Vec::new();
    let mut invalid = 0u32;

    for line in input.lines() {
        match parse_line(line) {
            Some(id) => ids.push(id),
            None => invalid += 1,
        }
    }

    print!("valid={} invalid={} ids=[", ids.len(), invalid);
    for (i, id) in ids.iter().enumerate() {
        if i > 0 {
            print!(",");
        }
        print!("{}", id);
    }
    print!("]");
}
