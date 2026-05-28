use std::cmp::Reverse;

#[derive(Debug)]
struct Team {
    name: &'static str,
    points: u32,
    goal_diff: i32,
    goals_scored: u32,
}

fn main() {
    let mut teams = vec![
        Team { name: "Arrows", points: 7, goal_diff: 2, goals_scored: 4 },
        Team { name: "Bees", points: 7, goal_diff: 4, goals_scored: 6 },
        Team { name: "Comets", points: 4, goal_diff: 1, goals_scored: 4 },
        Team { name: "Dynamos", points: 4, goal_diff: 1, goals_scored: 3 },
        Team { name: "Hawks", points: 7, goal_diff: 4, goals_scored: 5 },
    ];

    teams.sort_by_key(|t| (Reverse(t.points), t.name, Reverse(t.goal_diff), Reverse(t.goals_scored)));

    for (i, t) in teams.iter().enumerate() {
        println!(
            "{}. {} {} pts GD {:+} GS {}",
            i + 1,
            t.name,
            t.points,
            t.goal_diff,
            t.goals_scored
        );
    }
}
