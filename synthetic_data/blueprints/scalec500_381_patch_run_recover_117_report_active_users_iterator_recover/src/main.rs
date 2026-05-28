struct User {
    name: &'static str,
    enabled: bool,
    visits: u32,
    quota: Option<u32>,
}

fn build_report(users: &[User]) -> String {
    let mut total = 0u32;
    let lines: Vec<String> = users
        .iter()
        .filter(|u| u.enabled || u.quota.is_some())
        .map(|u| {
            let count = u.quota.unwrap_or(u.visits);
            total += count;
            format!("- {}: {}", u.name, count)
        })
        .collect();

    format!("active users:\n{}\nTOTAL={}", lines.join("\n"), total)
}

fn main() {
    let users = [
        User {
            name: "alice",
            enabled: true,
            visits: 5,
            quota: None,
        },
        User {
            name: "bob",
            enabled: false,
            visits: 4,
            quota: Some(2),
        },
        User {
            name: "carol",
            enabled: true,
            visits: 0,
            quota: Some(7),
        },
        User {
            name: "dave",
            enabled: true,
            visits: 10,
            quota: Some(0),
        },
    ];

    print!("{}", build_report(&users));
}
