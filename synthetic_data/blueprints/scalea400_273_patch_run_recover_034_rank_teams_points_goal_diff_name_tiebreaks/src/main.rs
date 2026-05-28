use std::cmp::Reverse;

#[derive(Debug, Clone)]
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

fn standings(mut teams: Vec<Team>) -> String {
    teams.sort_by_key(|t| (Reverse(t.points), Reverse(t.goal_diff()), t.name, Reverse(t.goals_for)));

    teams
        .iter()
        .enumerate()
        .map(|(i, t)| {
            format!(
                "{}. {} | {} pts | GD {:+} | GS {}",
                i + 1,
                t.name,
                t.points,
                t.goal_diff(),
                t.goals_for
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let teams = vec![
        Team { name: "Alpha", points: 7, goals_for: 4, goals_against: 2 },
        Team { name: "Bravo", points: 7, goals_for: 4, goals_against: 2 },
        Team { name: "Cobra", points: 6, goals_for: 3, goals_against: 2 },
        Team { name: "Delta", points: 6, goals_for: 4, goals_against: 3 },
        Team { name: "Echo", points: 4, goals_for: 2, goals_against: 2 },
        Team { name: "Frost", points: 4, goals_for: 2, goals_against: 3 },
    ];

    print!("{}", standings(teams));
}
