use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Team {
    city: &'static str,
    score: u32,
    wins: u32,
    losses: u32,
}

fn main() {
    let mut teams = vec![
        Team { city: "Dover", score: 17, wins: 4, losses: 0 },
        Team { city: "Birch", score: 17, wins: 5, losses: 2 },
        Team { city: "Cedar", score: 17, wins: 5, losses: 1 },
        Team { city: "Arbor", score: 17, wins: 5, losses: 1 },
        Team { city: "Elm", score: 15, wins: 6, losses: 3 },
        Team { city: "Flint", score: 15, wins: 6, losses: 4 },
    ];

    teams.sort_by(|a, b| {
        a.score.cmp(&b.score)
            .then_with(|| a.wins.cmp(&b.wins))
            .then_with(|| a.losses.cmp(&b.losses))
            .then(Ordering::Equal)
    });

    let output = teams
        .iter()
        .enumerate()
        .map(|(idx, t)| {
            format!(
                "{}. {} | score={} | wins={} | losses={}",
                idx + 1,
                t.city,
                t.score,
                t.wins,
                t.losses
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    print!("{}", output);
}
