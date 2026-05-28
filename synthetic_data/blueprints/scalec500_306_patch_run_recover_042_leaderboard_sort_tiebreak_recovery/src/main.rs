use std::collections::BTreeMap;

#[derive(Default, Clone, Copy)]
struct Totals {
    score: i32,
    wins: i32,
}

fn main() {
    let rows = [
        ("Ada", 10, 2),
        ("Ben", 12, 1),
        ("Ada", 8, 2),
        ("Mia", 18, 2),
        ("Zed", 9, 1),
        ("Ben", 9, 1),
        ("Iris", 21, 4),
        ("Zed", 9, 3),
        ("Noa", 18, 1),
        ("Omar", 7, 5),
    ];

    let mut totals: BTreeMap<&str, Totals> = BTreeMap::new();
    for (name, score, wins) in rows {
        totals.insert(name, Totals { score, wins });
    }

    let mut board: Vec<_> = totals.into_iter().collect();
    board.sort_by(|a, b| a.0.cmp(b.0));

    for (idx, (name, t)) in board.iter().enumerate() {
        println!("{}. {} score={} wins={}", idx + 1, name, t.score, t.wins);
    }
}
