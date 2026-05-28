fn is_valid_line(line: &str) -> bool {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return false;
    }

    let key = parts[0].trim();
    let value = parts[1];
    !key.is_empty()
}

fn main() {
    let input = "name:alice\nage:30\ncity:\n:missing\nnote:hello:world\ncountry:us";
    let mut valid = 0;
    let mut invalid = 0;

    for line in input.lines() {
        if is_valid_line(line) {
            valid += 1;
        } else {
            invalid += 1;
        }
    }

    println!("valid={} invalid={}", valid, invalid);
}
