use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Player {
    name: &'static str,
    score: u32,
    penalty: u32,
    wins: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Ana", score: 14, penalty: 3, wins: 5 },
        Player { name: "Bo", score: 14, penalty: 2, wins: 3 },
        Player { name: "Cy", score: 12, penalty: 1, wins: 6 },
        Player { name: "Di", score: 12, penalty: 1, wins: 2 },
        Player { name: "Eli", score: 14, penalty: 2, wins: 4 },
        Player { name: "Fay", score: 12, penalty: 4, wins: 4 },
    ];

    players.sort_by(|a, b| {
        a.score.cmp(&b.score)
            .then_with(|| a.penalty.cmp(&b.penalty))
            .then_with(|| a.wins.cmp(&b.wins))
            .then_with(|| b.name.cmp(&a.name))
    });

    let lines: Vec<String> = players
        .iter()
        .enumerate()
        .map(|(i, p)| {
            format!(
                "{}. {} score={} penalty={} wins={}",
                i + 1,
                p.name,
                p.score,
                p.penalty,
                p.wins
            )
        })
        .collect();

    println!("{}", lines.join("\n"));
}
