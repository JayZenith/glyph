fn parse_line(line: &str) -> Option<(String, String, i32)> {
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

    Some((id.to_string(), name.to_string(), score))
}

fn main() {
    let input = "ID-001|Ann|90\nID-002|Bob|70\nBAD|Eve|88\nID-003||50\nID-002|Bobby|71\nID-004|Dee|100\nID-005|Fox|101\nID-01A|Gia|40\nID-006|Hal|20|extra";
    let mut out = Vec::new();

    for line in input.lines() {
        if let Some((id, name, score)) = parse_line(line) {
            out.push(format!("{} {} {}", id, name, score));
        }
    }

    println!("{}", out.join("\n"));
}
