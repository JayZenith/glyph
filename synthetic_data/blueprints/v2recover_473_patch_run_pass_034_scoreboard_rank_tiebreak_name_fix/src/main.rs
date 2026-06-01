struct Player {
    name: &'static str,
    score: u32,
    penalties: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Zoe", score: 15, penalties: 2 },
        Player { name: "Ada", score: 15, penalties: 1 },
        Player { name: "Ian", score: 12, penalties: 3 },
        Player { name: "Bea", score: 12, penalties: 0 },
        Player { name: "Eli", score: 15, penalties: 1 },
    ];

    players.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .reverse()
            .then(a.penalties.cmp(&b.penalties).reverse())
            .then(a.name.cmp(b.name).reverse())
    });

    for (i, p) in players.iter().enumerate() {
        println!("{}. {} {} {}", i + 1, p.name, p.score, p.penalties);
    }
}
