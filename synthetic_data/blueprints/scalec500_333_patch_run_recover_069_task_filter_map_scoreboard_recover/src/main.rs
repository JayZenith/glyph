fn build_report(rows: &[&str]) -> String {
    rows.iter()
        .filter_map(|line| {
            let mut parts = line.split('|');
            let name = parts.next()?;
            let status = parts.next()?;
            let amount = parts.next()?.parse::<i32>().ok()?;
            if status == "ok" {
                Some((name.to_string(), amount))
            } else {
                None
            }
        })
        .filter(|(_, amount)| *amount >= 10)
        .map(|(name, amount)| format!("{}={}", name.to_lowercase(), amount))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let rows = [
        "ALPHA|ok|15",
        "BETA|skip|99",
        "GAMMA|ok|7",
        "DELTA|ok|13",
        "EPSILON|ok|9",
        "BROKEN|ok|x",
    ];

    print!("{}", build_report(&rows));
}
