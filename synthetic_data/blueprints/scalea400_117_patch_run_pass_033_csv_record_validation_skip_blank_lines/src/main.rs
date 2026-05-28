const INPUT: &str = "alice,42\n\nbob,7\ncarol,nope\ndave\n";

fn is_valid(line: &str) -> bool {
    let mut parts = line.split(',');
    let name = parts.next().unwrap_or("");
    let age = parts.next().unwrap_or("");
    parts.next().is_none() && !name.is_empty() && age.parse::<u32>().is_ok()
}

fn main() {
    let mut valid = 0;
    let mut invalid = 0;

    for line in INPUT.lines() {
        if is_valid(line) {
            valid += 1;
        } else {
            invalid += 1;
        }
    }

    println!("valid={} invalid={}", valid, invalid);
}
