struct Player {
    name: &'static str,
    points: u32,
    wins: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Zoe", points: 9, wins: 5 },
        Player { name: "Amy", points: 9, wins: 2 },
        Player { name: "Bob", points: 12, wins: 3 },
        Player { name: "Cara", points: 12, wins: 4 },
        Player { name: "Eli", points: 12, wins: 4 },
        Player { name: "Dana", points: 9, wins: 5 },
    ];

    players.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.wins.cmp(&b.wins))
            .then(a.name.cmp(&b.name))
    });

    for (i, p) in players.iter().enumerate() {
        println!("{}. {} {} pts ({}W)", i + 1, p.name, p.points, p.wins);
    }
}
