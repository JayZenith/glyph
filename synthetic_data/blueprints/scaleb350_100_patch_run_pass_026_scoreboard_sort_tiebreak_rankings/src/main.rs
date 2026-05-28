struct Player {
    name: &'static str,
    score: u32,
    penalty: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Alice", score: 90, penalty: 10 },
        Player { name: "Bob", score: 95, penalty: 8 },
        Player { name: "Cara", score: 95, penalty: 12 },
        Player { name: "Dan", score: 90, penalty: 5 },
        Player { name: "Eve", score: 95, penalty: 8 },
    ];

    players.sort_by(|a, b| {
        a.score.cmp(&b.score)
            .then(a.penalty.cmp(&b.penalty))
            .then(a.name.cmp(&b.name))
    });

    for (i, p) in players.iter().enumerate() {
        println!("{}. {} - {} pts (penalty {})", i + 1, p.name, p.score, p.penalty);
    }
}
