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

fn format_gd(diff: i32) -> String {
    if diff >= 0 {
        format!("+{}", diff)
    } else {
        diff.to_string()
    }
}

fn main() {
    let mut teams = vec![
        Team { name: "Lions", points: 9, goals_for: 8, goals_against: 5 },
        Team { name: "Tigers", points: 6, goals_for: 7, goals_against: 6 },
        Team { name: "Bears", points: 9, goals_for: 6, goals_against: 5 },
        Team { name: "Wolves", points: 6, goals_for: 4, goals_against: 5 },
        Team { name: "Hawks", points: 9, goals_for: 5, goals_against: 2 },
        Team { name: "Eagles", points: 6, goals_for: 3, goals_against: 6 },
    ];

    teams.sort_by_key(|t| (Reverse(t.points), t.name, Reverse(t.goal_diff())));

    for (idx, team) in teams.iter().enumerate() {
        println!(
            "{}. {} - {} pts (GD {})",
            idx + 1,
            team.name,
            team.points,
            format_gd(team.goal_diff())
        );
    }
}
