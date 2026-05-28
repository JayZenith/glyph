struct Team {
    name: &'static str,
    score: u32,
    penalty: u32,
    last: &'static str,
}

fn main() {
    let mut teams = vec![
        Team { name: "Cygnus", score: 17, penalty: 1, last: "10:05" },
        Team { name: "Dune", score: 17, penalty: 2, last: "09:10" },
        Team { name: "Astra", score: 17, penalty: 1, last: "09:40" },
        Team { name: "Ember", score: 15, penalty: 0, last: "08:55" },
        Team { name: "Boreal", score: 17, penalty: 1, last: "09:40" },
    ];

    teams.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(a.penalty.cmp(&b.penalty))
            .then(a.name.cmp(&b.name))
    });

    for (i, t) in teams.iter().enumerate() {
        println!(
            "{}. {:<6} score={} penalty={} last={}",
            i + 1,
            t.name,
            t.score,
            t.penalty,
            t.last
        );
    }
}
