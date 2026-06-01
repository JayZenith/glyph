fn parse_line(line: &str) -> Option<(String, u32)> {
    let mut parts = line.split(',');
    let name = parts.next()?.trim();
    let age = parts.next()?.trim().parse::<u32>().ok()?;
    Some((name.to_string(), age))
}

fn main() {
    let input = "Ada,34\nBob,xx\nCara,27\nDylan";
    let mut valid = Vec::new();

    for line in input.lines() {
        if let Some((name, age)) = parse_line(line) {
            valid.push(format!("{}:{}", name, age));
        }
    }

    println!("{} valid", valid.len());
    for row in valid {
        println!("{}", row);
    }
}
