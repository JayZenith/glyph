struct Player {
    name: &'static str,
    points: u32,
    wins: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Ben", points: 15, wins: 4 },
        Player { name: "Ava", points: 17, wins: 3 },
        Player { name: "Dax", points: 15, wins: 2 },
        Player { name: "Cara", points: 17, wins: 3 },
        Player { name: "Eli", points: 17, wins: 2 },
    ];

    players.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then(b.wins.cmp(&a.wins))
            .then(a.name.cmp(b.name))
    });

    for (i, p) in players.iter().enumerate() {
        println!("{}. {} - {} pts (wins: {})", i + 1, p.name, p.points, p.wins);
    }
}
