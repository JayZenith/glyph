fn parse_line(line: &str) -> Option<(String, u32)> {
    let mut parts = line.split('|');
    let name = parts.next()?.trim();
    let age_text = parts.next()?.trim();
    let score_text = parts.next()?.trim();

    if parts.next().is_some() || name.is_empty() {
        return None;
    }

    let age: i32 = age_text.parse().ok()?;
    let score: u32 = score_text.parse().ok()?;

    if age <= 0 || score > 100 {
        return None;
    }

    Some((name.to_string(), score))
}

fn main() {
    let input = "Ada|34|90\nBob|-2|88\nCara|22|101\n|44|70\nDan|x|40\nEve|29|100";

    let mut valid = 0usize;
    let mut invalid = 0usize;
    let mut names = Vec::new();

    for line in input.lines() {
        match parse_line(line) {
            Some((name, _score)) => {
                valid += 1;
                names.push(name);
            }
            None => invalid += 1,
        }
    }

    println!("valid={} invalid={} names={}", valid, invalid, names.join(","));
}
