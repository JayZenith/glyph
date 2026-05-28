fn parse_line(line: &str) -> Option<(&str, u32)> {
    let mut parts = line.split(',');
    let name = parts.next()?;
    let score = parts.next()?.parse::<u32>().ok()?;
    Some((name, score))
}

fn main() {
    let input = "Alice,42\nBob\nCara,notanumber\n,7\nDana,19";
    let mut valid = Vec::new();
    let mut invalid = 0;

    for line in input.lines() {
        match parse_line(line) {
            Some((name, score)) if !name.is_empty() => valid.push((name, score)),
            _ => invalid += 1,
        }
    }

    println!("valid: {}", valid.len());
    println!("invalid: {}", invalid);
    for (name, score) in valid {
        println!("{}:{}", name, score);
    }
}
