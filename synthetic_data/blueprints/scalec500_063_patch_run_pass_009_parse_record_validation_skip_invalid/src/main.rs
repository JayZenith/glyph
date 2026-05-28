fn parse_line(line: &str) -> Option<(String, u32)> {
    let (name, score_str) = line.split_once(':')?;
    let score = score_str.parse::<u32>().ok()?;
    Some((name.to_string(), score))
}

fn main() {
    let input = "alice:42\nbob:-5\ninvalid\ncara:7\ndave:101\n";
    let mut rows = Vec::new();

    for line in input.lines() {
        if let Some((name, score)) = parse_line(line) {
            rows.push((name, score));
        }
    }

    println!("accepted: {}", rows.len());
    for (name, score) in rows {
        println!("{}:{}", name, score);
    }
}
