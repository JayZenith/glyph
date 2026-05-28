use std::fmt::Write;

#[derive(Clone, Copy)]
struct Player {
    name: &'static str,
    score: u32,
    games: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Ada", score: 42, games: 5 },
        Player { name: "Bo", score: 42, games: 7 },
        Player { name: "Cy", score: 39, games: 4 },
        Player { name: "Dee", score: 35, games: 3 },
        Player { name: "Eli", score: 39, games: 9 },
    ];

    players.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(b.games.cmp(&a.games))
            .then(a.name.cmp(&b.name))
    });

    let mut out = String::new();
    let mut prev_score: Option<u32> = None;
    let mut rank = 0usize;
    let mut leader_score = 0u32;

    for p in players {
        if prev_score != Some(p.score) {
            rank += 1;
        }
        let gap = leader_score.saturating_sub(p.score);
        let _ = writeln!(
            out,
            "{}. {} score={} games={} gap={}",
            rank, p.name, p.score, p.games, gap
        );
        prev_score = Some(p.score);
        leader_score = p.score;
    }

    print!("{}", out.trim_end());
}
