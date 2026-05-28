#[derive(Clone)]
struct Team {
    name: &'static str,
    points: u32,
    goal_diff: i32,
}

fn main() {
    let mut teams = vec![
        Team { name: "Lions", points: 7, goal_diff: 3 },
        Team { name: "Hawks", points: 7, goal_diff: 1 },
        Team { name: "Bears", points: 7, goal_diff: 3 },
        Team { name: "Wolves", points: 4, goal_diff: 0 },
    ];

    teams.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.goal_diff.cmp(&b.goal_diff))
            .then(a.name.cmp(b.name))
    });

    for (idx, team) in teams.iter().enumerate() {
        println!(
            "{}. {} - {} pts ({:+})",
            idx + 1,
            team.name,
            team.points,
            team.goal_diff
        );
    }
}
