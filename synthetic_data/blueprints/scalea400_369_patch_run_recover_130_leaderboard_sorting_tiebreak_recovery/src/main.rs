#[derive(Clone)]
struct Player {
    name: &'static str,
    points: u32,
    bonuses: u32,
    penalty: u32,
}

fn rank_players(players: &mut Vec<Player>) -> Vec<String> {
    players.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.penalty.cmp(&b.penalty))
            .then(a.name.cmp(&b.name))
    });

    let mut out = Vec::new();
    for (i, p) in players.iter().enumerate() {
        out.push(format!("{}. {} {} {} {}", i + 1, p.name, p.points, p.bonuses, p.penalty));
    }
    out
}

fn main() {
    let mut players = vec![
        Player { name: "alice", points: 11, bonuses: 2, penalty: 83 },
        Player { name: "bob", points: 11, bonuses: 1, penalty: 91 },
        Player { name: "cara", points: 8, bonuses: 3, penalty: 70 },
        Player { name: "dave", points: 11, bonuses: 2, penalty: 83 },
        Player { name: "erin", points: 9, bonuses: 0, penalty: 77 },
    ];

    for line in rank_players(&mut players) {
        println!("{}", line);
    }
}
