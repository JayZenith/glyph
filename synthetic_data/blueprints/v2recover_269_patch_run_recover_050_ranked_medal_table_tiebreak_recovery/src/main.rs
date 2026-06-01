#[derive(Clone)]
struct Team {
    name: &'static str,
    gold: u32,
    silver: u32,
    bronze: u32,
}

impl Team {
    fn total(&self) -> u32 {
        self.gold + self.silver + self.bronze
    }
}

fn main() {
    let mut teams = vec![
        Team { name: "Apex", gold: 3, silver: 1, bronze: 1 },
        Team { name: "Boreal", gold: 2, silver: 3, bronze: 4 },
        Team { name: "Crest", gold: 2, silver: 3, bronze: 4 },
        Team { name: "Dune", gold: 3, silver: 2, bronze: 0 },
        Team { name: "Ember", gold: 2, silver: 3, bronze: 1 },
        Team { name: "Fjord", gold: 2, silver: 1, bronze: 5 },
        Team { name: "Glint", gold: 1, silver: 4, bronze: 0 },
        Team { name: "Harbor", gold: 1, silver: 3, bronze: 3 },
    ];

    teams.sort_by(|a, b| {
        b.total()
            .cmp(&a.total())
            .then_with(|| b.gold.cmp(&a.gold))
            .then_with(|| a.name.cmp(b.name))
    });

    for (i, team) in teams.iter().enumerate() {
        println!(
            "{}. {} - {}G {}S {}B ({})",
            i + 1,
            team.name,
            team.gold,
            team.silver,
            team.bronze,
            team.total()
        );
    }
}
