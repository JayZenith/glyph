struct Player {
    name: &'static str,
    score: u32,
    wins: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Ada", score: 17, wins: 5 },
        Player { name: "Bo", score: 17, wins: 4 },
        Player { name: "Gia", score: 14, wins: 7 },
        Player { name: "Eli", score: 14, wins: 7 },
        Player { name: "Dax", score: 14, wins: 6 },
        Player { name: "Fay", score: 12, wins: 9 },
        Player { name: "Cy", score: 12, wins: 3 },
    ];

    players.sort_by(|a, b| {
        b.score.cmp(&a.score)
            .then_with(|| a.wins.cmp(&b.wins))
            .then_with(|| b.name.cmp(&a.name))
    });

    let mut out = String::new();
    let mut rank = 1;
    for (i, p) in players.iter().enumerate() {
        if i > 0 && p.score != players[i - 1].score {
            rank += 1;
        }
        out.push_str(&format!("{}. {} {} pts ({} wins)\n", rank, p.name, p.score, p.wins));
    }

    print!("{}", out.trim_end());
}
