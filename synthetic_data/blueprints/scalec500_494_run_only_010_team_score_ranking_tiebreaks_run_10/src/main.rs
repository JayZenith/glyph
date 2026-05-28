#[derive(Clone, Copy)]
struct Team {
    name: &'static str,
    points: u32,
    goal_diff: i32,
    goals_for: u32,
}

fn main() {
    let mut teams = vec![
        Team { name: "Cobras", points: 14, goal_diff: 5, goals_for: 13 },
        Team { name: "Aardvarks", points: 12, goal_diff: 3, goals_for: 15 },
        Team { name: "Bears", points: 14, goal_diff: 7, goals_for: 11 },
        Team { name: "Dragons", points: 12, goal_diff: 8, goals_for: 10 },
        Team { name: "Falcons", points: 14, goal_diff: 7, goals_for: 12 },
        Team { name: "Eagles", points: 12, goal_diff: 8, goals_for: 9 },
    ];

    teams.sort_by(|a, b| {
        b.points.cmp(&a.points)
            .then_with(|| b.goal_diff.cmp(&a.goal_diff))
            .then_with(|| b.goals_for.cmp(&a.goals_for))
            .then_with(|| a.name.cmp(b.name))
    });

    for (idx, t) in teams.iter().enumerate() {
        println!(
            "{}. {} - {} pts (GD {:+}, GF {})",
            idx + 1,
            t.name,
            t.points,
            t.goal_diff,
            t.goals_for
        );
    }
}
