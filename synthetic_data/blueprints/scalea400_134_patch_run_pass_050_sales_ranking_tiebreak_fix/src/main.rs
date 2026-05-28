struct Team {
    name: &'static str,
    wins: u32,
    losses: u32,
}

fn main() {
    let mut teams = vec![
        Team { name: "North", wins: 14, losses: 7 },
        Team { name: "South", wins: 17, losses: 6 },
        Team { name: "East", wins: 17, losses: 5 },
        Team { name: "West", wins: 14, losses: 4 },
        Team { name: "Central", wins: 14, losses: 9 },
    ];

    teams.sort_by(|a, b| {
        a.wins
            .cmp(&b.wins)
            .then_with(|| a.losses.cmp(&b.losses))
            .then_with(|| a.name.cmp(&b.name))
    });

    for (idx, team) in teams.iter().enumerate() {
        println!(
            "{}. {} | {} wins | {} losses",
            idx + 1,
            team.name,
            team.wins,
            team.losses
        );
    }
}
