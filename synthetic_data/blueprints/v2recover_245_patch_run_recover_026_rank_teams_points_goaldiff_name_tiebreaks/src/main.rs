use std::cmp::Ordering;

#[derive(Clone, Debug)]
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
        Team { name: "Falcons", points: 7, goals_for: 5, goals_against: 2 },
        Team { name: "Owls", points: 7, goals_for: 4, goals_against: 1 },
        Team { name: "Tigers", points: 7, goals_for: 5, goals_against: 2 },
        Team { name: "Bears", points: 6, goals_for: 6, goals_against: 4 },
        Team { name: "Lynx", points: 6, goals_for: 5, goals_against: 3 },
        Team { name: "Sharks", points: 6, goals_for: 4, goals_against: 3 },
    ];

    teams.sort_by(|a, b| {
        b.points.cmp(&a.points)
            .then_with(|| a.goal_diff().cmp(&b.goal_diff()))
            .then_with(|| a.name.cmp(&b.name))
    });

    for (idx, team) in teams.iter().enumerate() {
        println!(
            "{}. {} {} pts GD {:+} GS {}",
            idx + 1,
            team.name,
            team.points,
            team.goal_diff(),
            team.goals_for
        );
    }
}
