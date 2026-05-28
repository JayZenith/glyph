use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Player {
    name: &'static str,
    points: u32,
    wins: u32,
    penalties: u32,
}

fn players() -> Vec<Player> {
    vec![
        Player { name: "Zed", points: 15, wins: 5, penalties: 4 },
        Player { name: "Ada", points: 12, wins: 6, penalties: 8 },
        Player { name: "Mira", points: 15, wins: 5, penalties: 4 },
        Player { name: "Nova", points: 15, wins: 5, penalties: 3 },
        Player { name: "Bea", points: 12, wins: 5, penalties: 2 },
        Player { name: "Lux", points: 12, wins: 6, penalties: 12 },
        Player { name: "Pax", points: 15, wins: 4, penalties: 1 },
    ]
}

fn sort_players(players: &mut [Player]) {
    players.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| a.penalties.cmp(&b.penalties))
            .then_with(|| a.wins.cmp(&b.wins))
            .then_with(|| b.name.cmp(&a.name))
    });
}

fn render(players: &[Player]) -> String {
    let mut out = String::new();
    for (idx, p) in players.iter().enumerate() {
        if idx > 0 {
            out.push('\n');
        }
        let rank = idx + 1;
        out.push_str(&format!(
            "{}. {} pts={} wins={} pen={} ",
            rank, p.name, p.points, p.wins, p.penalties
        ));
    }
    out
}

fn main() {
    let mut board = players();
    sort_players(&mut board);
    println!("{}", render(&board));
}
