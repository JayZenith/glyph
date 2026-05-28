struct Rep {
    name: &'static str,
    points: u32,
    wins: u32,
}

fn main() {
    let mut reps = vec![
        Rep { name: "Ada", points: 17, wins: 3 },
        Rep { name: "Bo", points: 15, wins: 5 },
        Rep { name: "Cy", points: 15, wins: 2 },
        Rep { name: "Dee", points: 12, wins: 6 },
        Rep { name: "Eli", points: 17, wins: 4 },
    ];

    reps.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.wins.cmp(&b.wins))
            .then(a.name.cmp(b.name))
    });

    for (i, rep) in reps.iter().enumerate() {
        println!("{}. {} - {} pts ({} wins)", i + 1, rep.name, rep.points, rep.wins);
    }
}
