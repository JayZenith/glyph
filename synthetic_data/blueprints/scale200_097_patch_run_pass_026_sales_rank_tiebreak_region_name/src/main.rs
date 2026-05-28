struct Rep {
    name: &'static str,
    region: &'static str,
    score: u32,
}

fn main() {
    let mut reps = vec![
        Rep { name: "Ann", region: "west", score: 9 },
        Rep { name: "Bob", region: "north", score: 12 },
        Rep { name: "Cara", region: "south", score: 12 },
        Rep { name: "Dan", region: "east", score: 12 },
        Rep { name: "Eve", region: "east", score: 9 },
    ];

    reps.sort_by(|a, b| {
        b.score.cmp(&a.score)
            .then(a.name.cmp(&b.name))
            .then(a.region.cmp(&b.region))
    });

    for (i, rep) in reps.iter().enumerate() {
        println!("{}. {} ({}) - {}", i + 1, rep.name, rep.region, rep.score);
    }
}
