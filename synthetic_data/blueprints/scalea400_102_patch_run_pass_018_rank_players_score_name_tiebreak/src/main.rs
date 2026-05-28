#[derive(Debug)]
struct Player {
    name: &'static str,
    score: u32,
    wins: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Ada", score: 9, wins: 3 },
        Player { name: "Bob", score: 12, wins: 2 },
        Player { name: "Cara", score: 12, wins: 4 },
        Player { name: "Dan", score: 9, wins: 5 },
        Player { name: "Eve", score: 12, wins: 4 },
    ];

    players.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .then(a.wins.cmp(&b.wins))
            .then(a.name.cmp(b.name))
    });

    for (idx, p) in players.iter().enumerate() {
        println!("{}. {} (score={}, wins={})", idx + 1, p.name, p.score, p.wins);
    }
}
