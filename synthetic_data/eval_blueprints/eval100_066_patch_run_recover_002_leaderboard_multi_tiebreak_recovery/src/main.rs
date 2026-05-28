#[derive(Clone, Copy)]
struct Team {
    name: &'static str,
    solved: u32,
    penalty: u32,
    last_accept: u32,
}

fn main() {
    let mut teams = vec![
        Team { name: "Ada", solved: 4, penalty: 460, last_accept: 180 },
        Team { name: "Bea", solved: 4, penalty: 500, last_accept: 150 },
        Team { name: "Cal", solved: 3, penalty: 300, last_accept: 120 },
        Team { name: "Dan", solved: 4, penalty: 460, last_accept: 180 },
        Team { name: "Eli", solved: 5, penalty: 350, last_accept: 180 },
        Team { name: "Fay", solved: 4, penalty: 500, last_accept: 220 },
        Team { name: "Gus", solved: 4, penalty: 460, last_accept: 200 },
    ];

    teams.sort_by(|a, b| {
        b.solved
            .cmp(&a.solved)
            .then(a.penalty.cmp(&b.penalty))
            .then(b.last_accept.cmp(&a.last_accept))
    });

    for (i, team) in teams.iter().take(5).enumerate() {
        println!(
            "{}. {} {} {} {}",
            i + 1,
            team.name,
            team.solved,
            team.penalty,
            team.last_accept
        );
    }
}
