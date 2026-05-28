use std::collections::{BTreeMap, HashSet};

fn main() {
    let input = "u1|Ada|20\nu2|Bob|\nu3||17\nu4|Cara|x\nu5|Ada|30\nu6|Eve|-1";
    let (accepted, counts) = process(input);
    println!("accepted: [{}]", accepted.join(", "));
    for key in ["missing_field", "invalid_age", "duplicate_name"] {
        println!("{}: {}", key, counts.get(key).copied().unwrap_or(0));
    }
}

fn process(input: &str) -> (Vec<String>, BTreeMap<&'static str, usize>) {
    let mut accepted = Vec::new();
    let mut counts = BTreeMap::new();
    let mut seen_names = HashSet::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 {
            *counts.entry("missing_field").or_insert(0) += 1;
            continue;
        }

        let id = parts[0];
        let name = parts[1];
        let age_text = parts[2];

        if id.is_empty() || name.is_empty() {
            *counts.entry("missing_field").or_insert(0) += 1;
            continue;
        }

        let age: i32 = age_text.parse().unwrap_or(0);
        if age < 0 {
            *counts.entry("invalid_age").or_insert(0) += 1;
            continue;
        }

        if !seen_names.insert(name.to_string()) {
            *counts.entry("duplicate_name").or_insert(0) += 1;
            continue;
        }

        accepted.push(id.to_string());
    }

    (accepted, counts)
}
