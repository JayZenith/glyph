struct User {
    name: &'static str,
    active: bool,
}

fn main() {
    let users = [
        User { name: "Liam", active: true },
        User { name: "Emma", active: false },
        User { name: "Ava", active: true },
        User { name: "Noah", active: true },
        User { name: "Mia", active: false },
    ];

    let names = users
        .iter()
        .filter(|u| u.active)
        .filter(|u| u.name.len() % 2 == 1)
        .map(|u| u.name)
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", names);
}
