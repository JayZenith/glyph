fn is_valid_record(line: &str) -> bool {
    let mut parts = line.split(',');
    let first = parts.next();
    let second = parts.next();

    if parts.next().is_some() {
        return false;
    }

    let Some(a) = first else { return false; };
    let Some(b) = second else { return false; };

    let a_ok = a.starts_with("id=") && !a[3..].is_empty() && a[3..].chars().all(|c| c.is_ascii_digit());
    let b_ok = b.starts_with("name=") && !b[5..].is_empty();

    a_ok || b_ok
}

fn main() {
    let input = "id=1,name=alice\nid=2,name=bob\nid=,name=carol\nname=dave,id=4\nid=5,name=\nid=x,name=erin";

    let mut valid = 0;
    let mut invalid = 0;

    for line in input.lines() {
        if is_valid_record(line) {
            valid += 1;
        } else {
            invalid += 1;
        }
    }

    println!("valid={} invalid={}", valid, invalid);
}
