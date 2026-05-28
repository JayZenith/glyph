struct Player {
    name: &'static str,
    score: u32,
    penalty: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Ana", score: 12, penalty: 5 },
        Player { name: "Bob", score: 15, penalty: 7 },
        Player { name: "Cara", score: 15, penalty: 4 },
        Player { name: "Dana", score: 15, penalty: 4 },
        Player { name: "Eve", score: 12, penalty: 3 },
    ];

    players.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .reverse()
            .then_with(|| b.penalty.cmp(&a.penalty))
            .then_with(|| b.name.cmp(a.name))
    });

    let out = players
        .iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} {} {}", i + 1, p.name, p.score, p.penalty))
        .collect::<Vec<_>>()
        .join("\n");

    print!("{}", out);
}
