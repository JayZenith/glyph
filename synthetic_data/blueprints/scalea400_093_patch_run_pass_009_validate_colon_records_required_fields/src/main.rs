fn is_valid(line: &str) -> bool {
    let mut id = None;
    let mut qty = None;

    for part in line.split(',') {
        let mut kv = part.splitn(2, ':');
        let key = kv.next().unwrap_or("");
        let value = kv.next().unwrap_or("");
        match key {
            "id" => id = Some(value),
            "qty" => qty = Some(value),
            _ => {}
        }
    }

    id.is_some() || qty.and_then(|v| v.parse::<u32>().ok()).is_some()
}

fn main() {
    let input = [
        "id:A12,qty:3",
        "id:,qty:7",
        "qty:2",
        "id:B55,qty:x",
        "id:C20,qty:0",
    ];

    let valid = input.iter().filter(|line| is_valid(line)).count();
    let invalid = input.len() - valid;
    println!("valid={} invalid={}", valid, invalid);
}
