struct Team {
    name: &'static str,
    wins: u32,
    draws: u32,
    goals_for: i32,
    goals_against: i32,
}

fn main() {
    let mut teams = vec![
        Team { name: "Oak", wins: 1, draws: 2, goals_for: 5, goals_against: 3 },
        Team { name: "Pine", wins: 1, draws: 2, goals_for: 4, goals_against: 2 },
        Team { name: "Birch", wins: 1, draws: 1, goals_for: 3, goals_against: 3 },
        Team { name: "Ash", wins: 0, draws: 4, goals_for: 2, goals_against: 2 },
        Team { name: "Elm", wins: 1, draws: 2, goals_for: 6, goals_against: 4 },
    ];

    teams.sort_by(|a, b| {
        let pa = a.wins * 3 + a.draws;
        let pb = b.wins * 3 + b.draws;
        let gda = a.goals_for - a.goals_against;
        let gdb = b.goals_for - b.goals_against;

        pb.cmp(&pa)
            .then(gdb.cmp(&gda))
            .then(b.goals_for.cmp(&a.goals_for))
            .then(a.name.cmp(b.name))
    });

    for (i, t) in teams.iter().enumerate() {
        let pts = t.wins * 3 + t.draws;
        let gd = t.goals_for - t.goals_against;
        println!(
            "{}. {} | {} pts | GF {} GA {} GD {}",
            i + 1,
            t.name,
            pts,
            t.goals_for,
            t.goals_against,
            gd
        );
    }
}
