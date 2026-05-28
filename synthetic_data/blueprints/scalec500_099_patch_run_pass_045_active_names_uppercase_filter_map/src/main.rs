struct User {
    name: &'static str,
    active: bool,
}

fn main() {
    let users = vec![
        User { name: "Ada", active: true },
        User { name: "", active: true },
        User { name: "Bob", active: false },
        User { name: "Cy", active: true },
    ];

    let out = users
        .iter()
        .filter(|u| u.active || !u.name.is_empty())
        .map(|u| u.name.to_uppercase())
        .collect::<Vec<_>>()
        .join(", ");

    println!("{}", out);
}
