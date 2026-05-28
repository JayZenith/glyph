fn valid_age(s: &str) -> bool {
    match s.parse::<u8>() {
        Ok(age) => (18..=99).contains(&age),
        Err(_) => false,
    }
}

fn valid_name(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_lowercase())
}

fn main() {
    let input = "alice,30,admin\nbob,17,user\ncarol,xx,staff\ndave,40\neve,22,ops";

    let mut valid = 0;
    let mut invalid = 0;
    let mut accepted_names = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            invalid += 1;
            continue;
        }

        let name = parts[0];
        let age = parts[1];

        if valid_name(name) && valid_age(age) {
            valid += 1;
            accepted_names.push(name);
        } else {
            invalid += 1;
        }
    }

    println!("valid: {}", valid);
    println!("invalid: {}", invalid);
    println!("accepted names: {}", accepted_names.join(","));
}
