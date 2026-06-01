struct Team {
    name: &'static str,
    points: u32,
    wins: u32,
    diff: i32,
}

fn main() {
    let mut teams = vec![
        Team { name: "Ava", points: 7, wins: 5, diff: 10 },
        Team { name: "Bob", points: 9, wins: 4, diff: 3 },
        Team { name: "Cara", points: 9, wins: 4, diff: 3 },
        Team { name: "Dan", points: 9, wins: 3, diff: 8 },
        Team { name: "Eve", points: 9, wins: 4, diff: 0 },
    ];

    teams.sort_by(|a, b| {
        b.points.cmp(&a.points)
            .then(b.wins.cmp(&a.wins))
            .then(b.diff.cmp(&a.diff))
            .then(a.name.cmp(&b.name))
    });

    for (i, t) in teams.iter().enumerate() {
        println!(
            "{}. {} - {} pts (wins: {}, diff: {})",
            i + 1,
            t.name,
            t.points,
            t.wins,
            t.diff
        );
    }
}
