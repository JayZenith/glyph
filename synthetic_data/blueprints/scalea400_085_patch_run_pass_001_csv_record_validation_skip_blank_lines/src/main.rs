fn is_valid(line: &str) -> bool {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 3 {
        return false;
    }

    let name = parts[0];
    let age = parts[1];
    let role = parts[2];

    !name.is_empty()
        && age.parse::<u8>().is_ok()
        && matches!(role, "admin" | "user")
}

fn main() {
    let input = "alice,30,admin\n\nbob,xx,user\ncarol,25,guest\ndave,41,user\n";
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
