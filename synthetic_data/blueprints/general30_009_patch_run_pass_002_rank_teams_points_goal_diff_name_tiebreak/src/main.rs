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
        Team { name: "Cobras", points: 7, goals_for: 5, goals_against: 3 },
        Team { name: "Bears", points: 7, goals_for: 6, goals_against: 4 },
        Team { name: "Aardvarks", points: 6, goals_for: 9, goals_against: 4 },
        Team { name: "Dragons", points: 4, goals_for: 3, goals_against: 4 },
        Team { name: "Falcons", points: 7, goals_for: 8, goals_against: 5 },
    ];

    teams.sort_by_key(|t| (Reverse(t.points), t.name, Reverse(t.goal_diff())));

    for (idx, team) in teams.iter().enumerate() {
        println!(
            "{}. {} - {} pts (gd {:+})",
            idx + 1,
            team.name,
            team.points,
            team.goal_diff()
        );
    }
}
