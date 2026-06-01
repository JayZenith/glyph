use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Entry {
    name: &'static str,
    team: &'static str,
    score: i32,
    active: bool,
}

fn main() {
    let entries = vec![
        Entry { name: "ivy", team: "blue", score: 30, active: true },
        Entry { name: "max", team: "red", score: 12, active: true },
        Entry { name: "zoe", team: "blue", score: 0, active: true },
        Entry { name: "ana", team: "red", score: -5, active: true },
        Entry { name: "eli", team: "blue", score: 12, active: false },
        Entry { name: "uma", team: "red", score: 3, active: true },
        Entry { name: "kai", team: "blue", score: 12, active: true },
    ];

    let kept: Vec<Entry> = entries
        .iter()
        .copied()
        .filter(|e| e.score >= 0)
        .collect();

    let accepted_total: i32 = kept.iter().map(|e| e.score).sum();

    let mut by_team: BTreeMap<&str, i32> = BTreeMap::new();
    for e in &kept {
        *by_team.entry(e.team).or_insert(0) += 1;
    }
    let by_team_line = by_team
        .iter()
        .map(|(team, total)| format!("{}={}", team, total))
        .collect::<Vec<_>>()
        .join(",");

    let top = kept
        .iter()
        .max_by_key(|e| e.score)
        .map(|e| format!("{}:{}", e.team, e.score))
        .unwrap_or_else(|| "none".to_string());

    println!("kept: {}", kept.len());
    println!("accepted_total: {}", accepted_total);
    println!("by_team: {}", by_team_line);
    println!("top: {}", top);
}
