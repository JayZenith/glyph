#[derive(Clone)]
struct Player {
    name: &'static str,
    score: u32,
    penalties: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Amy", score: 90, penalties: 1 },
        Player { name: "Bob", score: 95, penalties: 1 },
        Player { name: "Cara", score: 95, penalties: 2 },
        Player { name: "Dan", score: 90, penalties: 0 },
        Player { name: "Eve", score: 95, penalties: 1 },
    ];

    players.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .then(a.penalties.cmp(&b.penalties))
            .then(a.name.cmp(b.name))
    });

    for (i, p) in players.iter().enumerate() {
        println!("{}. {} - {} ({})", i + 1, p.name, p.score, p.penalties);
    }
}
