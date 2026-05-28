fn is_valid(line: &str) -> bool {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 3 {
        return false;
    }

    let id_ok = !parts[0].is_empty() && parts[0].chars().all(|c| c.is_ascii_digit());
    let name_ok = !parts[1].is_empty() && parts[1].chars().all(|c| c.is_ascii_alphabetic());
    let qty_ok = parts[2].parse::<u32>().is_ok();

    id_ok && name_ok && qty_ok
}

fn main() {
    let input = "101,apple,3\n\n102,banana,7\n103,,4\nxyz,pear,2\n";
    let mut valid = 0;
    let mut invalid = 0;

    for line in input.lines() {
        if is_valid(line) {
            valid += 1;
        } else {
            invalid += 1;
        }
    }

    println!("valid:{} invalid:{}", valid, invalid);
}
