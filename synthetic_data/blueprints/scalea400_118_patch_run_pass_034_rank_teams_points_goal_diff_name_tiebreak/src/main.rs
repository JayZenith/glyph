struct Team {
    name: &'static str,
    points: u32,
    goals_for: i32,
    goals_against: i32,
}

fn main() {
    let mut teams = vec![
        Team { name: "Tigers", points: 4, goals_for: 4, goals_against: 4 },
        Team { name: "Bears", points: 6, goals_for: 3, goals_against: 1 },
        Team { name: "Wolves", points: 4, goals_for: 3, goals_against: 3 },
        Team { name: "Lions", points: 6, goals_for: 4, goals_against: 2 },
    ];

    teams.sort_by(|a, b| {
        b.points.cmp(&a.points)
            .then_with(|| b.goals_for.cmp(&a.goals_for))
            .then_with(|| {
                let a_gd = a.goals_for - a.goals_against;
                let b_gd = b.goals_for - b.goals_against;
                b_gd.cmp(&a_gd)
            })
            .then_with(|| b.name.cmp(a.name))
    });

    for (idx, team) in teams.iter().enumerate() {
        let gd = team.goals_for - team.goals_against;
        println!(
            "{}. {} - {} pts, gd {}, gf {}",
            idx + 1,
            team.name,
            team.points,
            gd,
            team.goals_for
        );
    }
}
