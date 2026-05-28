struct Player {
    name: &'static str,
    score: Option<u32>,
    active: bool,
}

fn main() {
    let players = [
        Player { name: "Ava", score: Some(7), active: true },
        Player { name: "Bo", score: Some(4), active: false },
        Player { name: "Cy", score: Some(5), active: true },
        Player { name: "Di", score: Some(9), active: true },
        Player { name: "Eli", score: None, active: true },
    ];

    let report = players
        .iter()
        .filter_map(|p| p.score.map(|s| format!("{}={}", p.name, s)))
        .collect::<Vec<_>>()
        .join(", ");

    println!("{}", report);
}
