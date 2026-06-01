const INPUT: &str = "ID-001| Alice |90\nBAD|Eve|88\nID-002| |50\nID-003|Carl|101\nID-004|Dana|x\nID-005|Erin\nID-006|Frank|70|extra\nID-007| Bob |70\nID-01A|Gail|60\nID-010|Dora|100\n";

fn parse_line(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 3 {
        return None;
    }

    let id = parts[0].trim();
    let name = parts[1].trim();
    let score: i32 = parts[2].trim().parse().ok()?;

    if !id.starts_with("ID-") {
        return None;
    }
    if name.is_empty() {
        return None;
    }
    if !(0..100).contains(&score) {
        return None;
    }

    Some(format!("{}:{}:{}", id, name, score))
}

fn main() {
    let mut out = Vec::new();
    for line in INPUT.lines() {
        if let Some(row) = parse_line(line) {
            out.push(row);
        }
    }
    print!("{}", out.join("\n"));
}
