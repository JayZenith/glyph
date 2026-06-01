use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Player {
    name: &'static str,
    score: u32,
    wins: u32,
    penalties: u32,
}

fn leaderboard(mut players: Vec<Player>) -> String {
    players.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .then(a.wins.cmp(&b.wins))
            .then(a.penalties.cmp(&b.penalties))
            .then_with(|| b.name.cmp(a.name))
    });

    players
        .into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} | score={} wins={} penalties={}", i, p.name, p.score, p.wins, p.penalties))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let players = vec![
        Player { name: "Ada", score: 12, wins: 5, penalties: 2 },
        Player { name: "Bea", score: 12, wins: 4, penalties: 1 },
        Player { name: "Cy", score: 12, wins: 5, penalties: 4 },
        Player { name: "Eli", score: 12, wins: 5, penalties: 2 },
        Player { name: "Fay", score: 11, wins: 6, penalties: 3 },
    ];

    println!("{}", leaderboard(players));
}
