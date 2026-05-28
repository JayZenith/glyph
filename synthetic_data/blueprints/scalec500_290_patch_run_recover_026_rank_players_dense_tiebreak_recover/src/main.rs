use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Player {
    name: &'static str,
    score: u32,
    wins: u32,
}

fn best_per_name(players: &[Player]) -> Vec<Player> {
    let mut map: HashMap<&'static str, Player> = HashMap::new();
    for p in players {
        map.entry(p.name)
            .and_modify(|cur| {
                if p.score > cur.score {
                    *cur = p.clone();
                }
            })
            .or_insert_with(|| p.clone());
    }
    map.into_values().collect()
}

fn leaderboard(mut players: Vec<Player>) -> String {
    players.sort_by(|a, b| b.score.cmp(&a.score).then(a.name.cmp(&b.name)));

    let mut out = Vec::new();
    let mut prev_score = None;
    let mut rank = 0usize;

    for (i, p) in players.iter().enumerate() {
        if prev_score != Some(p.score) {
            rank = i + 1;
            prev_score = Some(p.score);
        }
        out.push(format!("{}. {} (score={}, wins={})", rank, p.name, p.score, p.wins));
    }

    out.join("\n")
}

fn main() {
    let players = vec![
        Player { name: "Ada", score: 88, wins: 9 },
        Player { name: "Bob", score: 90, wins: 5 },
        Player { name: "Cy", score: 90, wins: 5 },
        Player { name: "Dan", score: 85, wins: 6 },
        Player { name: "Eve", score: 85, wins: 7 },
        Player { name: "Ada", score: 90, wins: 6 },
    ];

    println!("{}", leaderboard(best_per_name(&players)));
}
