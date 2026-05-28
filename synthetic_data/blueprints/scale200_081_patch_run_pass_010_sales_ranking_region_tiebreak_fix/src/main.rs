struct Rep {
    name: &'static str,
    wins: u32,
    deals: u32,
    region: &'static str,
}

fn main() {
    let mut reps = vec![
        Rep { name: "Cy", wins: 7, deals: 31, region: "West" },
        Rep { name: "Ada", wins: 7, deals: 31, region: "East" },
        Rep { name: "Eli", wins: 5, deals: 42, region: "Central" },
        Rep { name: "Bo", wins: 7, deals: 29, region: "North" },
    ];

    reps.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then(a.deals.cmp(&b.deals))
            .then(a.name.cmp(&b.name))
    });

    let lines: Vec<String> = reps
        .iter()
        .enumerate()
        .map(|(idx, r)| format!("{}. {} | {} wins | {} deals | {}", idx + 1, r.name, r.wins, r.deals, r.region))
        .collect();

    print!("{}", lines.join("\n"));
}
