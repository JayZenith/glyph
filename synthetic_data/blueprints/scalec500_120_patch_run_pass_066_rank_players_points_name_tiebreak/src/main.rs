#[derive(Clone, Copy)]
struct Player {
    name: &'static str,
    points: u32,
    penalties: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Ben", points: 15, penalties: 2 },
        Player { name: "Ada", points: 15, penalties: 1 },
        Player { name: "Cara", points: 12, penalties: 3 },
        Player { name: "Dan", points: 12, penalties: 0 },
        Player { name: "Eve", points: 15, penalties: 1 },
    ];

    players.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.penalties.cmp(&b.penalties))
            .then(a.name.cmp(b.name))
    });

    for (idx, p) in players.iter().enumerate() {
        println!("{}. {} - {} pts ({} pen)", idx + 1, p.name, p.points, p.penalties);
    }
}
