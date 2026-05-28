const INPUT: &str = r#"
# ledger export
A1|debit|42|food,home
bad id|credit|10|ok
Z9|transfer|0|ops
B2|credit|7|tax,refund
C3|debit|-5|food
D4|other|9|misc
E5|credit|12|dup,dup
F6|debit|3|
G7|debit|+8|gear
H8|credit|11|MiXed
I9|credit|1|two,,parts
junk line
   # ignored comment
"#;

fn parse_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let parts: Vec<&str> = trimmed.split('|').collect();
    if parts.len() < 4 {
        return None;
    }

    let id = parts[0].trim();
    let kind = parts[1].trim();
    let amount = parts[2].trim();
    let tags = parts[3].trim();

    if id.is_empty() {
        return None;
    }

    if amount.parse::<i64>().is_err() {
        return None;
    }

    let tag_count = if tags.is_empty() {
        0
    } else {
        tags.split(',').count()
    };

    Some(format!("{}|{}|{}|{}", id, kind, amount, tag_count))
}

fn main() {
    let mut out = Vec::new();
    for line in INPUT.lines() {
        if let Some(row) = parse_line(line) {
            out.push(row);
        }
    }
    println!("{}", out.join("\n"));
}
