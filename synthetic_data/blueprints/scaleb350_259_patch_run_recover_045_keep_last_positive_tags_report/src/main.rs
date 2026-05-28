fn summarize(input: &str) -> String {
    let mut rows: Vec<(String, i32)> = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(':');
            let name = parts.next()?;
            let value = parts.next()?.parse::<i32>().ok()?;
            Some((name.to_string(), value))
        })
        .filter(|(_, value)| *value >= 0)
        .collect();

    rows.sort_by(|a, b| a.0.cmp(&b.0));

    rows.into_iter()
        .map(|(name, value)| format!("{}={}", name, value))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let data = "apple:3
banana:2
apple:-1
banana:6
carrot:0
date:-3
egg:bad
fig:4
apple:5";
    println!("{}", summarize(data));
}
