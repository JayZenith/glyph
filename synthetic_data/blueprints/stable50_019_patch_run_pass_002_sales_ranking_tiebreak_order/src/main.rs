#[derive(Clone, Copy)]
struct Player {
    name: &'static str,
    points: u32,
    wins: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Cy", points: 12, wins: 3 },
        Player { name: "Bo", points: 12, wins: 4 },
        Player { name: "Ada", points: 12, wins: 4 },
        Player { name: "Eli", points: 10, wins: 5 },
        Player { name: "Dee", points: 10, wins: 4 },
    ];

    players.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| b.wins.cmp(&a.wins))
    });

    for (idx, p) in players.iter().enumerate() {
        println!("{}. {} - {} pts ({} wins)", idx + 1, p.name, p.points, p.wins);
    }
}
