use std::cmp::Reverse;

#[derive(Debug)]
struct Team {
    name: &'static str,
    points: u32,
    goals_for: i32,
    goals_against: i32,
}

impl Team {
    fn goal_diff(&self) -> i32 {
        self.goals_for - self.goals_against
    }
}

fn main() {
    let mut teams = vec![
        Team { name: "Cobras", points: 10, goals_for: 6, goals_against: 1 },
        Team { name: "Aardvarks", points: 10, goals_for: 6, goals_against: 1 },
        Team { name: "Bears", points: 10, goals_for: 7, goals_against: 2 },
        Team { name: "Falcons", points: 10, goals_for: 8, goals_against: 2 },
        Team { name: "Dragons", points: 8, goals_for: 9, goals_against: 1 },
        Team { name: "Eagles", points: 8, goals_for: 7, goals_against: -1 },
    ];

    teams.sort_by_key(|t| (Reverse(t.points), t.name));

    for (idx, team) in teams.iter().enumerate() {
        println!(
            "{}. {} {} pts GD {:+} GF {}",
            idx + 1,
            team.name,
            team.points,
            team.goal_diff(),
            team.goals_for
        );
    }
}
