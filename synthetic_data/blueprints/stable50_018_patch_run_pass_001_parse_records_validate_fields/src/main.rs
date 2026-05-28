fn parse_and_count(input: &str) -> (usize, usize) {
    let mut valid = 0;
    let mut invalid = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split('|');
        let name = parts.next().unwrap_or("");
        let age_str = parts.next().unwrap_or("");
        let city = parts.next().unwrap_or("");

        let age_ok = age_str.parse::<u32>().is_ok();
        let name_ok = !name.is_empty();
        let city_ok = !city.is_empty();
        let structure_ok = parts.next().is_none();

        if age_ok && name_ok && city_ok && structure_ok {
            valid += 1;
        } else {
            invalid += 1;
        }
    }

    (valid, invalid)
}

fn main() {
    let input = "alice|30|Paris\nbob||Rome\n|22|Milan\ncarol|xx|Berlin\ndave|41|Lisbon";
    let (valid, invalid) = parse_and_count(input);
    println!("valid={} invalid={}", valid, invalid);
}
