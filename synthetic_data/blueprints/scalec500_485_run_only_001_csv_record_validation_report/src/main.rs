fn valid_id(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

fn valid_name(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic() || c == ' ')
}

fn valid_age(s: &str) -> bool {
    match s.parse::<u8>() {
        Ok(n) => (1..=120).contains(&n),
        Err(_) => false,
    }
}

fn main() {
    let input = "1,Alice,30\n2,Bob,-1\n3,Carol,27\n4,,22\n5,Eve,200";

    let mut valid = 0;
    let mut invalid_ids = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            continue;
        }

        let id = parts[0];
        let name = parts[1];
        let age = parts[2];

        if valid_id(id) && valid_name(name) && valid_age(age) {
            valid += 1;
        } else {
            invalid_ids.push(id);
        }
    }

    println!("valid: {}", valid);
    println!("invalid: {}", invalid_ids.len());
    println!("invalid ids: {}", invalid_ids.join(","));
}
