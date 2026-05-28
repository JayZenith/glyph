use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Team {
    name: &'static str,
    points: u32,
    wins: u32,
    losses: u32,
}

fn teams() -> Vec<Team> {
    vec![
        Team { name: "Gamma", points: 9, wins: 3, losses: 1 },
        Team { name: "Alpha", points: 9, wins: 3, losses: 0 },
        Team { name: "Beta", points: 7, wins: 2, losses: 2 },
        Team { name: "Delta", points: 7, wins: 2, losses: 1 },
        Team { name: "Epsilon", points: 4, wins: 1, losses: 4 },
        Team { name: "Beta", points: 6, wins: 2, losses: 3 },
        Team { name: "Zeta", points: 4, wins: 1, losses: 3 },
    ]
}

fn rank_teams(mut teams: Vec<Team>) -> Vec<Team> {
    teams.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.wins.cmp(&b.wins))
            .then(a.losses.cmp(&b.losses))
            .then_with(|| b.name.cmp(a.name))
    });
    teams
}

fn render(teams: &[Team]) -> String {
    teams
        .iter()
        .enumerate()
        .map(|(i, t)| format!("{}. {} - {} pts ({}-{})", i + 1, t.name, t.points, t.wins, t.losses))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let ranked = rank_teams(teams());
    println!("{}", render(&ranked));
}
