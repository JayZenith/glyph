struct Player {
    name: &'static str,
    wins: u32,
    scored: i32,
    allowed: i32,
}

fn main() {
    let mut players = vec![
        Player { name: "Ava", wins: 4, scored: 22, allowed: 14 },
        Player { name: "Ben", wins: 4, scored: 19, allowed: 11 },
        Player { name: "Cy", wins: 3, scored: 18, allowed: 13 },
        Player { name: "Dax", wins: 3, scored: 24, allowed: 12 },
        Player { name: "Eli", wins: 4, scored: 17, allowed: 7 },
    ];

    players.sort_by(|a, b| {
        a.wins
            .cmp(&b.wins)
            .then_with(|| diff(a).cmp(&diff(b)))
            .then_with(|| b.name.cmp(a.name))
    });

    for (idx, p) in players.iter().enumerate() {
        println!(
            "{}. {} (wins: {}, diff: {})",
            idx + 1,
            p.name,
            p.wins,
            diff(p)
        );
    }
}

fn diff(p: &Player) -> i32 {
    p.scored - p.allowed
}
