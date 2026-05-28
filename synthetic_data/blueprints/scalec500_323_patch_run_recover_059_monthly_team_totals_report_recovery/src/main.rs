use std::collections::BTreeMap;

fn main() {
    let data = [
        ("2024-01", "alpha", 5),
        ("2024-01", "beta", 2),
        ("2024-01", "alpha", -1),
        ("2024-02", "beta", 3),
        ("2024-02", "alpha", 0),
        ("2024-02", "gamma", 4),
        ("2024-01", "beta", 3),
        ("2024-02", "beta", 2),
        ("2024-01", "alpha", 3),
        ("2024-01", "delta", -2),
    ];

    let mut months: BTreeMap<&str, BTreeMap<&str, i32>> = BTreeMap::new();
    for (month, team, delta) in data {
        *months.entry(month).or_default().entry(team).or_default() += delta;
    }

    let mut out = Vec::new();
    for (month, teams) in months {
        out.push(month.to_string());
        let mut rows: Vec<_> = teams.into_iter().collect();
        rows.sort_by(|a, b| a.0.cmp(b.0));
        for (team, total) in rows {
            if total >= 0 {
                out.push(format!("{}: {}", team, total));
            }
        }
    }

    print!("{}", out.join("\n"));
}
