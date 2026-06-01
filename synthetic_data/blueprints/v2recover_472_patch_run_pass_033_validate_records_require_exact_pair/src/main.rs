fn parse_line(line: &str) -> Option<(&str, i32)> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 2 {
        return None;
    }
    let name = parts[0];
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let score = parts[1].parse::<i32>().ok()?;
    if !(0..=100).contains(&score) {
        return None;
    }
    Some((name, score))
}

fn main() {
    let input = "Ada:42\nBob:7:extra\n:10\nMax:101\nEve:100\nR2D2:50\nNoScore:";
    let mut valid = Vec::new();

    for line in input.lines() {
        if let Some((name, score)) = parse_line(line) {
            valid.push(format!("{}={}", name, score));
        }
    }

    for row in &valid {
        println!("{}", row);
    }
    println!("valid={}", valid.len());
}
