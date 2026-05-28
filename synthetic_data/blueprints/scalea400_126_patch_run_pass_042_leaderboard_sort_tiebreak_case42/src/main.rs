struct Player {
    name: &'static str,
    score: u32,
    wins: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Zoe", score: 12, wins: 6 },
        Player { name: "Ava", score: 17, wins: 5 },
        Player { name: "Mia", score: 17, wins: 4 },
        Player { name: "Ben", score: 12, wins: 3 },
        Player { name: "Eli", score: 17, wins: 5 },
    ];

    players.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .then_with(|| a.wins.cmp(&b.wins))
            .then_with(|| a.name.cmp(b.name))
    });

    for (i, p) in players.iter().enumerate() {
        println!("{}. {} - {} pts ({} wins)", i + 1, p.name, p.score, p.wins);
    }
}
