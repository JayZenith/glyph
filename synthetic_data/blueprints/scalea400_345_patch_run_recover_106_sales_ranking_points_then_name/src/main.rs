#[derive(Clone)]
struct Player {
    name: &'static str,
    points: u32,
    wins: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Zoe", points: 9, wins: 2 },
        Player { name: "Ava", points: 9, wins: 3 },
        Player { name: "Ian", points: 7, wins: 1 },
        Player { name: "Mia", points: 9, wins: 2 },
        Player { name: "Eli", points: 7, wins: 4 },
    ];

    players.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| a.name.cmp(&b.name))
    });

    let mut lines = Vec::new();
    let mut rank = 1;
    for i in 0..players.len() {
        if i > 0 && players[i].points != players[i - 1].points {
            rank += 1;
        }
        lines.push(format!(
            "{}. {} - {} pts ({} wins)",
            rank, players[i].name, players[i].points, players[i].wins
        ));
    }

    println!("{}", lines.join("\n"));
}
