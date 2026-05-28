fn parse_line(line: &str) -> Option<(&str, u32)> {
    let (name, score_text) = line.split_once('|')?;
    if name.is_empty() {
        return None;
    }
    let score = score_text.parse::<u32>().unwrap_or(0);
    Some((name, score))
}

fn main() {
    let input = "alice|10\nbob|x\n|7\ncarol|5\ndave-9";
    let mut names = Vec::new();
    let mut total = 0u32;

    for line in input.lines() {
        if let Some((name, score)) = parse_line(line) {
            names.push(name);
            total += score;
        }
    }

    println!("valid={} total={} names={}", names.len(), total, names.join(","));
}
