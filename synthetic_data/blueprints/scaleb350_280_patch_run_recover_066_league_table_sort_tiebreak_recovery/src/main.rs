struct TeamStat {
    name: &'static str,
    points: u32,
    goals_for: i32,
    goals_against: i32,
}

impl TeamStat {
    fn goal_diff(&self) -> i32 {
        self.goals_for - self.goals_against
    }
}

fn main() {
    let mut table = vec![
        TeamStat { name: "Falcons", points: 6, goals_for: 4, goals_against: 2 },
        TeamStat { name: "Hawks", points: 6, goals_for: 5, goals_against: 3 },
        TeamStat { name: "Cobras", points: 6, goals_for: 6, goals_against: 4 },
        TeamStat { name: "Lions", points: 4, goals_for: 3, goals_against: 3 },
        TeamStat { name: "Bears", points: 4, goals_for: 3, goals_against: 3 },
        TeamStat { name: "Sharks", points: 0, goals_for: 1, goals_against: 7 },
    ];

    table.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| b.name.cmp(&a.name))
    });

    let lines: Vec<String> = table
        .iter()
        .enumerate()
        .map(|(i, t)| {
            format!(
                "{}. {} - {} pts, GD {:+}, GS {}",
                i + 1,
                t.name,
                t.points,
                t.goal_diff(),
                t.goals_for
            )
        })
        .collect();

    print!("{}", lines.join("\n"));
}
