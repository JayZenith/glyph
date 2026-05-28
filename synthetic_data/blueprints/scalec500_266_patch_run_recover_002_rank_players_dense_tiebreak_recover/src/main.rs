use std::cmp::Reverse;

#[derive(Clone, Copy)]
struct Player {
    name: &'static str,
    score: u32,
    wins: u32,
}

fn main() {
    let mut players = vec![
        Player { name: "zoe", score: 19, wins: 5 },
        Player { name: "amy", score: 19, wins: 5 },
        Player { name: "ivy", score: 19, wins: 7 },
        Player { name: "ben", score: 17, wins: 3 },
        Player { name: "max", score: 17, wins: 9 },
        Player { name: "noa", score: 12, wins: 8 },
    ];

    players.sort_by_key(|p| (Reverse(p.score), p.name, Reverse(p.wins)));

    let mut out = Vec::new();
    for (i, p) in players.iter().enumerate() {
        let rank = i + 1;
        out.push(format!("{}. {:<4} {} pts {} wins", rank, p.name, p.score, p.wins));
    }

    print!("{}", out.join("\n"));
}
