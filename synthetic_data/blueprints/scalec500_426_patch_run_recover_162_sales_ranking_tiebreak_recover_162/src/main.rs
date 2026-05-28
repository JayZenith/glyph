#[derive(Clone)]
struct Entry {
    name: &'static str,
    score: u32,
    wins: u32,
}

fn main() {
    let mut items = vec![
        Entry { name: "Zoe", score: 12, wins: 8 },
        Entry { name: "Ava", score: 17, wins: 4 },
        Entry { name: "Mia", score: 12, wins: 5 },
        Entry { name: "Eli", score: 17, wins: 6 },
        Entry { name: "Ben", score: 17, wins: 6 },
    ];

    items.sort_by(|a, b| {
        b.score.cmp(&a.score)
            .then(a.name.cmp(b.name))
            .then(b.wins.cmp(&a.wins))
    });

    let lines: Vec<String> = items
        .iter()
        .enumerate()
        .map(|(i, e)| format!("{}. {} - {} pts ({} wins)", i + 1, e.name, e.score, e.wins))
        .collect();

    print!("{}", lines.join("\n"));
}
