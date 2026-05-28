struct Player {
    name: &'static str,
    points: u32,
    penalties: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Zoe", points: 15, penalties: 2 },
        Player { name: "Ana", points: 12, penalties: 0 },
        Player { name: "Bob", points: 15, penalties: 2 },
        Player { name: "Eli", points: 12, penalties: 3 },
        Player { name: "Cara", points: 15, penalties: 1 },
    ];

    players.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.penalties.cmp(&b.penalties))
            .then(a.name.cmp(&b.name))
    });

    for (i, p) in players.iter().enumerate() {
        println!("{}. {} - {} pts ({} pen)", i + 1, p.name, p.points, p.penalties);
    }
}
