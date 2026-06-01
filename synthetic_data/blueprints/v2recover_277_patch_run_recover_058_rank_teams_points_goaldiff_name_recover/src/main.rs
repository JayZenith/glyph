use std::cmp::Reverse;

#[derive(Clone, Debug)]
struct Team {
    name: &'static str,
    played: u32,
    wins: u32,
    draws: u32,
    gf: i32,
    ga: i32,
}

impl Team {
    fn points(&self) -> u32 {
        self.wins * 3
    }

    fn gd(&self) -> i32 {
        self.ga - self.gf
    }
}

fn main() {
    let mut teams = vec![
        Team { name: "Aces", played: 5, wins: 1, draws: 2, gf: 4, ga: 4 },
        Team { name: "Bears", played: 4, wins: 2, draws: 2, gf: 6, ga: 4 },
        Team { name: "Cobras", played: 5, wins: 2, draws: 1, gf: 8, ga: 7 },
        Team { name: "Hawks", played: 4, wins: 2, draws: 2, gf: 7, ga: 4 },
    ];

    teams.sort_by_key(|t| {
        (
            Reverse(t.points()),
            Reverse(t.gf),
            t.name,
        )
    });

    println!("Pos Team Pts GF GA GD");
    for (i, t) in teams.iter().enumerate() {
        println!(
            "{} {} {} {} {} {}",
            i + 1,
            t.name,
            t.played,
            t.gf,
            t.ga,
            t.gd()
        );
    }
}
