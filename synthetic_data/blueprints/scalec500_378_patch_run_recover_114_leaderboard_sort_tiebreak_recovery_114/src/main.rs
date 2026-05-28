#[derive(Clone, Debug)]
struct Team {
    name: &'static str,
    solved: u32,
    penalty: u32,
    last_accept: u32,
}

fn main() {
    let mut teams = vec![
        Team { name: "Ivy", solved: 7, penalty: 430, last_accept: 90 },
        Team { name: "Zoe", solved: 6, penalty: 350, last_accept: 80 },
        Team { name: "Ada", solved: 7, penalty: 410, last_accept: 95 },
        Team { name: "Moe", solved: 7, penalty: 410, last_accept: 102 },
        Team { name: "Eli", solved: 7, penalty: 410, last_accept: 95 },
    ];

    teams.sort_by(|a, b| {
        b.solved
            .cmp(&a.solved)
            .then(a.penalty.cmp(&b.penalty))
            .then(a.name.cmp(&b.name))
    });

    for (i, t) in teams.iter().enumerate() {
        let rank = i + 1;
        println!(
            "{}. {} solved={} penalty={} last={}",
            rank, t.name, t.solved, t.penalty, t.last_accept
        );
    }
}
