fn is_valid(line: &str) -> bool {
    let mut id_ok = false;
    let mut qty_ok = false;

    for part in line.split(';') {
        if let Some((key, value)) = part.split_once('=') {
            match key {
                "id" => {
                    id_ok = !value.is_empty();
                }
                "qty" => {
                    qty_ok = value.parse::<u32>().is_ok();
                }
                _ => {}
            }
        }
    }

    id_ok || qty_ok
}

fn main() {
    let input = "id=A1;qty=3\nid=;qty=8\nqty=2\nid=B7;qty=no\nid=C3;qty=0";

    let mut valid = 0;
    let mut invalid = 0;

    for line in input.lines() {
        if is_valid(line) {
            valid += 1;
        } else {
            invalid += 1;
        }
    }

    println!("valid={} invalid={}", valid, invalid);
}
