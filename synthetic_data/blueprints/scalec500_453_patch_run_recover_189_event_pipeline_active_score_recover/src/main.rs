use std::collections::{HashMap, HashSet};

fn main() {
    let active_users: HashSet<&str> = ["zoe", "amy", "bob"].into_iter().collect();
    let events = [
        "zoe|view|5",
        "amy|click|0",
        "bob|click|7",
        "bob|click|10",
        "mia|click|100",
        "zoe|click|3",
        "zoe|click|-2",
        "amy|click|4",
        "amy|oops",
        "zoe|click|8",
        "bob|click|bad",
    ];

    let mut totals: HashMap<&str, i32> = HashMap::new();

    events
        .iter()
        .filter_map(|line| {
            let mut parts = line.split('|');
            let user = parts.next()?;
            let kind = parts.next()?;
            let score = parts.next()?.parse::<i32>().ok()?;
            Some((user, kind, score))
        })
        .filter(|(_, kind, _)| *kind == "click")
        .for_each(|(user, _, score)| {
            *totals.entry(user).or_insert(0) += score;
        });

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));

    let out = rows
        .into_iter()
        .map(|(user, total)| format!("{}:{}", user, total))
        .collect::<Vec<_>>()
        .join("\n");

    print!("{}", out);
}
