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
        Team { name: "Falcons", points: 7, goals_for: 8, goals_against: 4 },
        Team { name: "Aces", points: 6, goals_for: 5, goals_against: 2 },
        Team { name: "Bears", points: 6, goals_for: 6, goals_against: 3 },
        Team { name: "Cobras", points: 6, goals_for: 6, goals_against: 3 },
        Team { name: "Dragons", points: 4, goals_for: 3, goals_against: 3 },
    ];

    teams.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| b.goals_for.cmp(&a.goals_for))
            .then_with(|| a.name.cmp(&b.name))
    });

    for (i, t) in teams.iter().enumerate() {
        println!(
            "{}. {} {} pts GD {:+} GF {}",
            i + 1,
            t.name,
            t.points,
            t.goal_diff(),
            t.goals_for
        );
    }
}
