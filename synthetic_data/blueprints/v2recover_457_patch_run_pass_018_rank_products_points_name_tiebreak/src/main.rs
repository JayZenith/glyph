struct Item {
    name: &'static str,
    score: i32,
}

fn main() {
    let mut items = vec![
        Item { name: "Gamma", score: 9 },
        Item { name: "Alpha", score: 12 },
        Item { name: "Epsilon", score: 12 },
        Item { name: "Delta", score: 17 },
        Item { name: "Beta", score: 17 },
    ];

    items.sort_by(|a, b| a.score.cmp(&b.score));

    let mut out = Vec::new();
    let mut rank = 0;
    let mut prev_score = None;

    for (idx, item) in items.iter().enumerate() {
        if prev_score != Some(item.score) {
            rank = idx + 1;
            prev_score = Some(item.score);
        }
        out.push(format!("{}. {} ({})", rank, item.name, item.score));
    }

    println!("{}", out.join("\n"));
}
