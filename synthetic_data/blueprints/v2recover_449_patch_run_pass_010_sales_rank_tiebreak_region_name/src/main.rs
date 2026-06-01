struct Rep {
    name: &'static str,
    region: &'static str,
    score: u32,
}

fn main() {
    let mut reps = vec![
        Rep { name: "Zoe", region: "West", score: 19 },
        Rep { name: "Ian", region: "East", score: 17 },
        Rep { name: "Ava", region: "East", score: 19 },
        Rep { name: "Eli", region: "North", score: 17 },
        Rep { name: "Mia", region: "North", score: 19 },
    ];

    reps.sort_by(|a, b| b.score.cmp(&a.score).then(a.name.cmp(&b.name)));

    for (i, rep) in reps.iter().enumerate() {
        println!("{}. {} [{}] - {}", i + 1, rep.name, rep.region, rep.score);
    }
}
