struct Row {
    id: u32,
    score: Option<i32>,
    tags: &'static [&'static str],
}

fn main() {
    let rows = [
        Row { id: 1, score: Some(10), tags: &["keep", "core"] },
        Row { id: 2, score: Some(5), tags: &["skip"] },
        Row { id: 3, score: None, tags: &["keep"] },
        Row { id: 4, score: Some(7), tags: &["keep", "bonus"] },
        Row { id: 5, score: Some(12), tags: &["bonus", "keep"] },
        Row { id: 6, score: Some(4), tags: &["keep"] },
    ];

    let kept: Vec<(u32, i32)> = rows
        .iter()
        .filter(|row| row.tags.iter().any(|tag| *tag == "keep"))
        .filter_map(|row| row.score.map(|score| (row.id, score)))
        .filter(|(_, score)| *score > 4)
        .map(|(id, score)| (id, score + 1))
        .collect();

    let total: i32 = kept.iter().map(|(_, score)| *score).sum();
    let ids = kept
        .iter()
        .map(|(id, _)| id.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("kept={} total={} ids={}", kept.len(), total, ids);
}
