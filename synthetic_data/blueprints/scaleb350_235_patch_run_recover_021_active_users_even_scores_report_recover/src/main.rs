fn main() {
    let users = [
        ("amy", true, Some(4)),
        ("bob", false, Some(10)),
        ("cid", true, Some(8)),
        ("dia", true, None),
        ("eli", true, Some(3)),
    ];

    let mut total = 0;
    let lines: Vec<String> = users
        .iter()
        .filter_map(|(name, active, score)| {
            score.map(|s| {
                total += s;
                format!("{}={}", name, s)
            })
        })
        .collect();

    println!("{}", lines.join("\n"));
    println!("total={}", total);
}
