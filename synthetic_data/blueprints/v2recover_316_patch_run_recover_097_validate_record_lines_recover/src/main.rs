const INPUT: &str = "ID-001|A|42\nID-77|A|42\nID-002|C|55\nID-003|B|9\nID-004|B|100\nID-005|A|\nX-006|A|22\nID-007|B|17|extra\nID-010|B|10\nID-120|A|99\n";

fn valid_id(id: &str) -> bool {
    id.starts_with("ID-") && id[3..].chars().all(|c| c.is_ascii_digit())
}

fn valid_kind(kind: &str) -> bool {
    kind == "A" || kind == "B"
}

fn valid_value(value: &str) -> bool {
    value.parse::<u32>().map(|n| n >= 10 && n <= 99).unwrap_or(false)
}

fn main() {
    let mut out = Vec::new();

    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 3 {
            continue;
        }

        let id = parts[0];
        let kind = parts[1];
        let value = parts[2];

        if valid_id(id) && valid_kind(kind) && valid_value(value) {
            out.push(id.to_string());
        }
    }

    println!("{}", out.join("\n"));
}
