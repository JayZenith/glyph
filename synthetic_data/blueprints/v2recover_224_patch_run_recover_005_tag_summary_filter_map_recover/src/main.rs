struct User {
    id: u32,
    active: bool,
    tags: &'static [&'static str],
}

fn summarize(users: &[User]) -> String {
    users
        .iter()
        .filter(|u| u.active)
        .filter_map(|u| {
            let tags: Vec<String> = u
                .tags
                .iter()
                .map(|t| t.to_ascii_uppercase())
                .collect();
            if tags.is_empty() {
                None
            } else {
                Some(format!("{}:{}", u.id, tags.join(",")))
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let users = [
        User {
            id: 1,
            active: true,
            tags: &["a", " ", "b", "a"],
        },
        User {
            id: 2,
            active: false,
            tags: &["x", "y"],
        },
        User {
            id: 3,
            active: true,
            tags: &["", "b", "c", "b"],
        },
    ];

    println!("{}", summarize(&users));
}
