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
        Team { name: "Arrows", points: 7, goals_for: 5, goals_against: 2 },
        Team { name: "Bears", points: 7, goals_for: 4, goals_against: 3 },
        Team { name: "Cobras", points: 7, goals_for: 6, goals_against: 3 },
        Team { name: "Dynamos", points: 4, goals_for: 3, goals_against: 3 },
        Team { name: "Eagles", points: 4, goals_for: 2, goals_against: 2 },
    ];

    teams.sort_by_key(|t| (Reverse(t.points), Reverse(t.goal_diff()), t.name, Reverse(t.goals_for)));

    for (idx, team) in teams.iter().enumerate() {
        println!(
            "{}. {} {} pts gd {} gs {}",
            idx + 1,
            team.name,
            team.points,
            team.goal_diff(),
            team.goals_for
        );
    }
}
