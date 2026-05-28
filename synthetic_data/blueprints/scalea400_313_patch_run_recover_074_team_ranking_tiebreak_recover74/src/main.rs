#[derive(Clone, Copy)]
struct Team {
    name: &'static str,
    points: u32,
    goals_for: u32,
    goals_against: u32,
}

impl Team {
    fn gd(&self) -> i32 {
        self.goals_for as i32 - self.goals_against as i32
    }
}

fn main() {
    let mut teams = vec![
        Team { name: "Bears", points: 9, goals_for: 6, goals_against: 2 },
        Team { name: "Hawks", points: 9, goals_for: 6, goals_against: 2 },
        Team { name: "Cobras", points: 9, goals_for: 5, goals_against: 1 },
        Team { name: "Falcons", points: 7, goals_for: 4, goals_against: 3 },
        Team { name: "Eagles", points: 7, goals_for: 3, goals_against: 2 },
        Team { name: "Dragons", points: 7, goals_for: 5, goals_against: 5 },
    ];

    teams.sort_by(|a, b| {
        b.points.cmp(&a.points)
            .then_with(|| b.goals_for.cmp(&a.goals_for))
            .then_with(|| b.gd().cmp(&a.gd()))
            .then_with(|| a.name.cmp(&b.name))
    });

    let mut lines = Vec::new();
    for (i, t) in teams.iter().enumerate() {
        let rank = i + 1;
        lines.push(format!(
            "{}. {} {} pts gd:{:+} gs:{}",
            rank,
            t.name,
            t.points,
            t.gd(),
            t.goals_for
        ));
    }

    println!("{}", lines.join("\n"));
}
