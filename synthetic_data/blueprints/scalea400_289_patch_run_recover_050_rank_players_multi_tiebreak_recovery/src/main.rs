#[derive(Clone, Copy)]
struct Player {
    name: &'static str,
    score: u32,
    penalty: u32,
}

fn leaderboard(players: &mut Vec<Player>) -> String {
    players.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(a.name.cmp(&b.name))
            .then(a.penalty.cmp(&b.penalty))
    });

    let mut lines = Vec::new();
    let mut current_rank = 1usize;

    for (i, p) in players.iter().enumerate() {
        if i > 0 {
            let prev = players[i - 1];
            if p.score != prev.score || p.penalty != prev.penalty {
                current_rank += 1;
            }
        }
        lines.push(format!(
            "{}. {} score={} penalty={}",
            current_rank, p.name, p.score, p.penalty
        ));
    }

    lines.join("\n")
}

fn main() {
    let mut players = vec![
        Player { name: "Zoe", score: 10, penalty: 1 },
        Player { name: "Ava", score: 12, penalty: 3 },
        Player { name: "Ben", score: 10, penalty: 2 },
        Player { name: "Mia", score: 12, penalty: 5 },
        Player { name: "Eli", score: 12, penalty: 3 },
        Player { name: "Kai", score: 8, penalty: 0 },
        Player { name: "Ian", score: 10, penalty: 2 },
    ];

    println!("{}", leaderboard(&mut players));
}
