#[derive(Clone, Debug)]
struct Team {
    name: &'static str,
    points: u32,
    goal_diff: i32,
    goals_scored: u32,
}

fn main() {
    let mut teams = vec![
        Team { name: "Cobras", points: 10, goal_diff: 2, goals_scored: 8 },
        Team { name: "Bears", points: 10, goal_diff: 4, goals_scored: 7 },
        Team { name: "Arrows", points: 8, goal_diff: 3, goals_scored: 5 },
        Team { name: "Falcons", points: 10, goal_diff: 4, goals_scored: 9 },
        Team { name: "Eclipse", points: 8, goal_diff: 3, goals_scored: 6 },
        Team { name: "Dynamos", points: 8, goal_diff: 3, goals_scored: 6 },
    ];

    teams.sort_by(|a, b| {
        b.points.cmp(&a.points)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| b.goal_diff.cmp(&a.goal_diff))
            .then_with(|| b.goals_scored.cmp(&a.goals_scored))
    });

    for (i, team) in teams.iter().enumerate() {
        println!(
            "{}. {} {} pts GD {:+} GS {}",
            i + 1,
            team.name,
            team.points,
            team.goal_diff,
            team.goals_scored
        );
    }
}
