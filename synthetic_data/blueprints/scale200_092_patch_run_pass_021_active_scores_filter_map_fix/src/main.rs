struct User {
    name: &'static str,
    active: bool,
    score: Option<u32>,
}

fn main() {
    let users = [
        User { name: "Ana", active: true, score: Some(12) },
        User { name: "Ben", active: false, score: Some(99) },
        User { name: "Cora", active: true, score: None },
        User { name: "Dee", active: true, score: Some(8) },
        User { name: "Eli", active: false, score: None },
        User { name: "Fay", active: true, score: Some(15) },
    ];

    let output = users
        .iter()
        .filter_map(|u| if !u.active { u.score } else { None })
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", output);
}
