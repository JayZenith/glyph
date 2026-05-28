#[derive(Clone)]
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

fn format_gd(n: i32) -> String {
    if n >= 0 {
        format!("+{}", n)
    } else {
        n.to_string()
    }
}

fn main() {
    let mut teams = vec![
        Team { name: "Rockets", points: 7, goals_for: 6, goals_against: 2 },
        Team { name: "Aces", points: 7, goals_for: 7, goals_against: 4 },
        Team { name: "Falcons", points: 9, goals_for: 8, goals_against: 3 },
        Team { name: "Blizzard", points: 7, goals_for: 5, goals_against: 1 },
        Team { name: "Comets", points: 7, goals_for: 6, goals_against: 2 },
        Team { name: "Dynamos", points: 6, goals_for: 4, goals_against: 2 },
    ];

    teams.sort_by(|a, b| {
        b.points.cmp(&a.points)
            .then_with(|| b.gd().cmp(&a.gd()))
            .then_with(|| a.name.cmp(b.name))
    });

    for (i, team) in teams.iter().enumerate() {
        println!(
            "{}. {} {} pts gd {} gs {}",
            i + 1,
            team.name,
            team.points,
            format_gd(team.gd()),
            team.goals_for
        );
    }
}
