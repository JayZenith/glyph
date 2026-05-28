const INPUT: &str = "Ava,17,blue\nBo,-1,red\nCara,30\nDan,22,\nEli,xx,green\nMia,20,teal\nZoe,18,blue,extra\n";

fn parse_valid(line: &str) -> Option<(String, u32)> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return None;
    }

    let name = parts[0].trim();
    let age: u32 = parts[1].trim().parse().ok()?;
    let color = parts[2].trim();

    if name.len() < 2 {
        return None;
    }

    if color.is_empty() {
        return Some((name.to_string(), age));
    }

    Some((name.to_string(), age))
}

fn main() {
    let mut out = Vec::new();
    for line in INPUT.lines() {
        if let Some((name, age)) = parse_valid(line) {
            out.push(format!("{}:{}", name, age));
        }
    }
    print!("{}", out.join("\n"));
}
