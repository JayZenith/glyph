use std::collections::HashMap;

const INPUT: &str = "1|zoe|9\n2|amy|17\n3|bob|17\n4|AL|22\n5||4\n6|mia|101\n7|ivy|x\n8|amy|12\n9|sam|7|extra\nabc|neo|5\n\n";

fn main() {
    let mut best: HashMap<String, u32> = HashMap::new();

    for line in INPUT.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 3 {
            continue;
        }

        let name = parts[1];
        if name.is_empty() {
            continue;
        }

        let score: u32 = match parts[2].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        best.insert(name.to_string(), score);
    }

    let mut rows: Vec<(String, u32)> = best.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0));

    for (name, score) in rows {
        println!("{}:{}", name, score);
    }
}
