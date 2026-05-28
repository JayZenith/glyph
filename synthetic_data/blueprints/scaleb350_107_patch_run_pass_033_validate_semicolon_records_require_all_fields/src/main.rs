fn valid_record(line: &str) -> bool {
    let mut id_ok = false;
    let mut name_ok = false;
    let mut age_ok = false;

    for part in line.split(';') {
        let mut it = part.splitn(2, '=');
        let key = it.next().unwrap_or("");
        let value = it.next().unwrap_or("");
        match key {
            "id" => id_ok = !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()),
            "name" => name_ok = !value.is_empty(),
            "age" => {
                age_ok = value
                    .parse::<u32>()
                    .map(|n| n <= 120)
                    .unwrap_or(false)
            }
            _ => {}
        }
    }

    id_ok || name_ok || age_ok
}

fn main() {
    let input = "id=101;name=Ana;age=30\nid=;name=Bob;age=44\nid=102;name=;age=50\nid=103;name=Cia;age=130\nid=104;name=Dee;age=29";

    let mut valid = 0;
    let mut invalid = 0;

    for line in input.lines() {
        if valid_record(line) {
            valid += 1;
        } else {
            invalid += 1;
        }
    }

    println!("valid={} invalid={}", valid, invalid);
}
