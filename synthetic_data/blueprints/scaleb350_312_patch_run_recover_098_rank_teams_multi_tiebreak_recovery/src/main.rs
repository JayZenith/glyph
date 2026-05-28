struct Team {
    name: &'static str,
    points: u32,
    goals_for: i32,
    goals_against: i32,
}

impl Team {
    fn gd(&self) -> i32 {
        self.goals_for - self.goals_against
    }
}

fn main() {
    let mut teams = vec![
        Team { name: "Cobras", points: 7, goals_for: 4, goals_against: 1 },
        Team { name: "Alphas", points: 7, goals_for: 4, goals_against: 1 },
        Team { name: "Dynamos", points: 7, goals_for: 5, goals_against: 3 },
        Team { name: "Bees", points: 7, goals_for: 5, goals_against: 2 },
    ];

    teams.sort_by(|a, b| {
        a.points.cmp(&b.points)
            .then_with(|| a.gd().cmp(&b.gd()))
            .then_with(|| a.name.cmp(b.name))
    });

    for (i, t) in teams.iter().enumerate() {
        println!(
            "{}. {} - {} pts, gd {}, gs {}",
            i + 1,
            t.name,
            t.points,
            t.gd(),
            t.goals_for
        );
    }
}
