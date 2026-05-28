use std::collections::HashSet;

fn main() {
    let input = "1|Alice|42|true\n2|Bob|x|false\n1|Cara|55|true\n3|Dan|77|maybe\n4|Eve|90\n5||12|true\n6|Frank|100|false";

    let mut seen = HashSet::new();
    let mut valid = 0;
    let mut invalid = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let line_no = idx + 1;
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() != 4 {
            invalid.push(format!("line {}: wrong field count", line_no));
            continue;
        }

        let id = parts[0];
        let name = parts[1];
        let score = parts[2];
        let active = parts[3];

        if name.is_empty() {
            invalid.push(format!("line {}: empty name", line_no));
            continue;
        }

        if !seen.insert(id) {
            invalid.push(format!("line {}: duplicate id", line_no));
            continue;
        }

        if score.parse::<u32>().ok().filter(|n| *n <= 100).is_none() {
            invalid.push(format!("line {}: bad score", line_no));
            continue;
        }

        if active != "true" && active != "false" {
            invalid.push(format!("line {}: bad active", line_no));
            continue;
        }

        valid += 1;
    }

    let mut out = format!("valid:{} invalid:{}", valid, invalid.len());
    for msg in invalid {
        out.push('\n');
        out.push_str(&msg);
    }
    print!("{}", out);
}
