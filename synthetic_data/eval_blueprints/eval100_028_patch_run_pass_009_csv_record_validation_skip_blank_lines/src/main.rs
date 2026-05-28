const INPUT: &str = "alice,42\n\n bob,7\ncarol,x\ndave\n";

fn is_valid_record(line: &str) -> bool {
    let parts: Vec<_> = line.split(',').collect();
    if parts.len() != 2 {
        return false;
    }

    let name = parts[0].trim();
    let score = parts[1].trim();

    !name.is_empty() && score.parse::<u32>().is_ok()
}

fn main() {
    let mut valid = 0;
    let mut invalid = 0;

    for line in INPUT.lines() {
        if is_valid_record(line) {
            valid += 1;
        } else {
            invalid += 1;
        }
    }

    println!("valid={} invalid={}", valid, invalid);
}
