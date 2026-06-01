use std::collections::HashSet;

const INPUT: &str = "id=A1;score=7;active=true
score=5;active=false
id=A1;score=3;active=yes
id=B2;score=11;active=false
id=C3;score=2;active=true;extra=1";

fn validate_line(line: &str, seen: &HashSet<String>) -> String {
    let mut id = None;
    let mut score = None;
    let mut active = None;

    for part in line.split(';') {
        let mut kv = part.splitn(2, '=');
        let key = kv.next().unwrap_or("");
        let value = kv.next().unwrap_or("");
        match key {
            "id" => id = Some(value.to_string()),
            "score" => score = value.parse::<i32>().ok(),
            "active" => active = Some(value == "true" || value == "false"),
            _ => {}
        }
    }

    if id.is_none() {
        return "missing id".to_string();
    }
    let idv = id.unwrap();
    if seen.contains(&idv) {
        return format!("duplicate id {}", idv);
    }
    if score.is_none() || !(0..10).contains(&score.unwrap()) {
        return format!("bad score {}", score.unwrap_or(-1));
    }
    if active != Some(true) {
        return "bad active".to_string();
    }
    "ok".to_string()
}

fn main() {
    let mut seen = HashSet::new();
    let mut out = Vec::new();

    for (idx, line) in INPUT.lines().enumerate() {
        let verdict = validate_line(line, &seen);
        if verdict == "ok" {
            let id = line.split(';').find_map(|p| p.strip_prefix("id=")).unwrap().to_string();
            seen.insert(id);
            out.push(format!("valid: {}", idx + 1));
        } else {
            out.push(format!("invalid: line {} {}", idx + 1, verdict));
        }
    }

    println!("{}", out.join("\n"));
}
