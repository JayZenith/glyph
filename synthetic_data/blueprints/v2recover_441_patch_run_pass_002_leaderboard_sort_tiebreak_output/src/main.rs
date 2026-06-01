#[derive(Clone, Copy)]
struct Player {
    name: &'static str,
    solved: u32,
    penalty: u32,
    last_accept: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "Zoe", solved: 5, penalty: 110, last_accept: 9 },
        Player { name: "Ava", solved: 5, penalty: 90, last_accept: 11 },
        Player { name: "Ian", solved: 4, penalty: 70, last_accept: 12 },
        Player { name: "Mia", solved: 4, penalty: 70, last_accept: 8 },
        Player { name: "Eli", solved: 5, penalty: 90, last_accept: 14 },
        Player { name: "Nia", solved: 4, penalty: 80, last_accept: 7 },
    ];

    players.sort_by(|a, b| {
        a.solved
            .cmp(&b.solved)
            .then(a.penalty.cmp(&b.penalty))
            .then(a.last_accept.cmp(&b.last_accept))
            .then(a.name.cmp(&b.name))
    });

    for (idx, p) in players.iter().enumerate() {
        println!(
            "{}. {} | solved={} penalty={} last={}",
            idx + 1,
            p.name,
            p.solved,
            p.penalty,
            p.last_accept
        );
    }
}
