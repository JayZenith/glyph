struct Player {
    name: &'static str,
    score: u32,
    penalties: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Bob", score: 120, penalties: 5 },
        Player { name: "Ada", score: 120, penalties: 2 },
        Player { name: "Cy", score: 90, penalties: 1 },
        Player { name: "Dan", score: 120, penalties: 4 },
        Player { name: "Eve", score: 120, penalties: 2 },
    ];

    players.sort_by(|a, b| {
        a.score.cmp(&b.score)
            .then(a.penalties.cmp(&b.penalties))
            .then(a.name.cmp(&b.name))
    });

    for (i, p) in players.iter().enumerate() {
        println!("{}. {} {} {}", i + 1, p.name, p.score, p.penalties);
    }
}
