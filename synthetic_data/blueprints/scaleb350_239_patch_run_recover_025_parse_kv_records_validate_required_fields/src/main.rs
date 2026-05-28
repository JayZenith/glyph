fn is_valid_record(line: &str) -> Option<String> {
    let mut id: Option<String> = None;
    let mut qty: Option<i32> = None;
    let mut status: Option<String> = None;

    for part in line.split(';') {
        let Some((k, v)) = part.split_once('=') else {
            continue;
        };
        match k {
            "id" => id = Some(v.to_string()),
            "qty" => qty = v.parse::<i32>().ok(),
            "status" => status = Some(v.to_string()),
            _ => {}
        }
    }

    match (id, qty, status) {
        (Some(id), Some(qty), Some(status)) if qty >= 0 || status == "ok" => Some(id),
        _ => None,
    }
}

fn main() {
    let input = "id=alpha;qty=3;status=ok
id=beta;qty=0;status=hold
id=gamma;qty=2;status=bad
qty=7;status=ok
id=delta;qty=5;status=hold
id=eps;qty=abc;status=ok";

    let mut valid = 0;
    let mut invalid = 0;
    let mut ids = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        if let Some(id) = is_valid_record(line) {
            valid += 1;
            ids.push(id);
        } else {
            invalid += 1;
        }
    }

    println!("valid={} invalid={}", valid, invalid);
    println!("ids={}", ids.join(","));
}
