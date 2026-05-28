#[derive(Clone, Debug)]
struct Team {
    name: &'static str,
    solved: u32,
    penalty: u32,
}

fn standings(mut teams: Vec<Team>) -> String {
    teams.sort_by(|a, b| {
        a.solved
            .cmp(&b.solved)
            .then_with(|| a.penalty.cmp(&b.penalty))
            .then_with(|| b.name.cmp(&a.name))
    });

    let mut out = Vec::new();
    for (i, t) in teams.iter().enumerate() {
        out.push(format!(
            "{}. {} | solved={} penalty= {}",
            i + 1,
            t.name,
            t.solved,
            t.penalty
        ));
    }
    out.join("\n").replace("penalty= ", "penalty=")
}

fn main() {
    let teams = vec![
        Team {
            name: "Alpha",
            solved: 3,
            penalty: 250,
        },
        Team {
            name: "Beta",
            solved: 3,
            penalty: 250,
        },
        Team {
            name: "Gamma",
            solved: 3,
            penalty: 400,
        },
        Team {
            name: "Delta",
            solved: 4,
            penalty: 300,
        },
        Team {
            name: "Epsilon",
            solved: 2,
            penalty: 150,
        },
    ];

    println!("{}", standings(teams));
}
