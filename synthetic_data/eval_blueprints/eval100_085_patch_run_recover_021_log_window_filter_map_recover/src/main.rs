fn parse_line(line: &str) -> Option<(u32, &str, &str, &str)> {
    let mut parts = line.split('|');
    let ts = parts.next()?.parse().ok()?;
    let user = parts.next()?;
    let status = parts.next()?;
    let tags = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    Some((ts, user, status, tags))
}

fn main() {
    let input = [
        "100|alice|ok|build,green",
        "101|bob|fail|lint",
        "102|alice|ok|deploy",
        "103|alice|skip|build",
        "104|carol|ok|ops",
        "105|bob|ok|build,unit",
        "106|alice|fail|smoke",
        "99|dave|ok|legacy",
        "bad|erin|ok|misc",
        "107|carol|skip|ops,late",
    ];

    let start = 100;
    let end = 106;
    let users = ["alice", "bob", "carol"];

    let mut out = Vec::new();
    for user in users {
        let rows: Vec<_> = input
            .iter()
            .filter_map(|line| parse_line(line))
            .filter(|(ts, name, _, _)| *ts > start && *ts < end && *name == user)
            .collect();

        let ok = rows.iter().filter(|(_, _, status, _)| *status == "ok").count();
        let fail = rows.iter().filter(|(_, _, status, _)| *status == "fail").count();

        let mut tags: Vec<&str> = rows
            .iter()
            .filter_map(|(_, _, status, tags)| {
                if *status == "ok" {
                    Some(*tags)
                } else {
                    None
                }
            })
            .flat_map(|tags| tags.split(','))
            .collect();
        tags.sort();
        tags.dedup();

        out.push(format!(
            "{}: {} ok / {} fail / {} tags [{}]",
            user,
            ok,
            fail,
            tags.len(),
            tags.join(",")
        ));
    }

    println!("{}", out.join("\n"));
}
