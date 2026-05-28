#[derive(Clone, Copy)]
struct Player {
    name: &'static str,
    wins: u32,
    losses: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Bea", wins: 12, losses: 2 },
        Player { name: "Ada", wins: 12, losses: 1 },
        Player { name: "Dan", wins: 10, losses: 3 },
        Player { name: "Cy", wins: 10, losses: 0 },
        Player { name: "Eli", wins: 12, losses: 1 },
    ];

    players.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then(a.name.cmp(b.name))
            .then(b.losses.cmp(&a.losses))
    });

    let mut prev_wins = None;
    let mut rank = 0usize;

    for (i, p) in players.iter().enumerate() {
        if prev_wins != Some(p.wins) {
            rank = i + 1;
            prev_wins = Some(p.wins);
        }
        println!("{}. {} {}-{}", rank, p.name, p.wins, p.losses);
    }
}
