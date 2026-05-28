use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Team {
    name: &'static str,
    points: u32,
    gf: u32,
    ga: u32,
}

impl Team {
    fn gd(&self) -> i32 {
        self.gf as i32 - self.ga as i32
    }
}

fn standings(mut teams: Vec<Team>) -> String {
    teams.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| b.gf.cmp(&a.gf))
            .then_with(|| a.name.cmp(&b.name))
    });

    let mut out = String::from("Rank Team Pts GD GF\n");
    for (i, t) in teams.iter().enumerate() {
        out.push_str(&format!("{} {} {} {} {}", i + 1, t.name, t.points, t.gd(), t.gf));
        if i + 1 < teams.len() {
            out.push('\n');
        }
    }
    out
}

fn main() {
    let teams = vec![
        Team { name: "Bears", points: 9, gf: 7, ga: 3 },
        Team { name: "Lynx", points: 7, gf: 4, ga: 1 },
        Team { name: "Owls", points: 7, gf: 5, ga: 2 },
        Team { name: "Hawks", points: 7, gf: 5, ga: 2 },
        Team { name: "Tigers", points: 6, gf: 6, ga: 4 },
    ];

    println!("{}", standings(teams));
}
