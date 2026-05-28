struct Racer {
    name: &'static str,
    points: u32,
    wins: u32,
}

fn main() {
    let mut racers = vec![
        Racer { name: "Alpha", points: 15, wins: 6 },
        Racer { name: "Beta", points: 18, wins: 4 },
        Racer { name: "Gamma", points: 15, wins: 6 },
        Racer { name: "Delta", points: 18, wins: 6 },
        Racer { name: "Epsilon", points: 15, wins: 3 },
    ];

    racers.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| b.wins.cmp(&a.wins))
            .then_with(|| b.name.cmp(&a.name))
    });

    for (idx, racer) in racers.iter().enumerate() {
        println!(
            "{}. {} - {} pts ({} wins)",
            idx + 1,
            racer.name,
            racer.points,
            racer.wins
        );
    }
}
