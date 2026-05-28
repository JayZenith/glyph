#[derive(Clone, Debug)]
struct Team {
    name: &'static str,
    solved: u32,
    penalty: u32,
}

fn main() {
    let mut teams = vec![
        Team { name: "Ada", solved: 7, penalty: 95 },
        Team { name: "Bob", solved: 7, penalty: 110 },
        Team { name: "Cy", solved: 6, penalty: 80 },
        Team { name: "Dan", solved: 6, penalty: 70 },
        Team { name: "Eve", solved: 7, penalty: 95 },
        Team { name: "Fay", solved: 6, penalty: 80 },
        Team { name: "Gus", solved: 5, penalty: 60 },
    ];

    teams.sort_by(|a, b| {
        a.solved
            .cmp(&b.solved)
            .then(a.penalty.cmp(&b.penalty))
            .then(a.name.cmp(&b.name))
    });

    let mut out = Vec::new();
    for (i, t) in teams.iter().enumerate() {
        out.push(format!(
            "{}. {} solved={} penalty={}",
            i + 1,
            t.name,
            t.solved,
            t.penalty
        ));
    }

    print!("{}", out.join("\n"));
}
