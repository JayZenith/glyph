struct Team {
    name: &'static str,
    wins: u32,
    draws: u32,
    losses: u32,
    goals_for: i32,
    goals_against: i32,
}

impl Team {
    fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }

    fn goal_diff(&self) -> i32 {
        self.goals_for - self.goals_against
    }
}

fn main() {
    let mut teams = vec![
        Team { name: "Beta", wins: 5, draws: 2, losses: 1, goals_for: 11, goals_against: 9 },
        Team { name: "Gamma", wins: 4, draws: 3, losses: 1, goals_for: 10, goals_against: 6 },
        Team { name: "Alpha", wins: 5, draws: 2, losses: 1, goals_for: 13, goals_against: 5 },
        Team { name: "Epsilon", wins: 4, draws: 3, losses: 1, goals_for: 9, goals_against: 5 },
        Team { name: "Delta", wins: 5, draws: 2, losses: 1, goals_for: 12, goals_against: 4 },
    ];

    teams.sort_by(|a, b| {
        b.points()
            .cmp(&a.points())
            .then_with(|| b.goal_diff().cmp(&a.goal_diff()))
            .then_with(|| b.goals_for.cmp(&a.goals_for))
            .then_with(|| a.name.cmp(&b.name))
    });

    let lines: Vec<String> = teams
        .iter()
        .enumerate()
        .map(|(i, t)| {
            format!(
                "{}. {} - {} pts (diff {:+}, scored {})",
                i + 1,
                t.name,
                t.points(),
                t.goal_diff(),
                t.goals_for
            )
        })
        .collect();

    print!("{}", lines.join("\n"));
}
