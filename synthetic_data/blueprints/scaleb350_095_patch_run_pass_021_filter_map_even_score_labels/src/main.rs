struct User {
    name: &'static str,
    score: i32,
    active: bool,
}

fn main() {
    let users = [
        User { name: "Ada", score: 9, active: true },
        User { name: "Bo", score: 12, active: true },
        User { name: "Cy", score: 14, active: false },
        User { name: "Dee", score: 18, active: true },
        User { name: "Eli", score: 11, active: true },
    ];

    let out = users
        .iter()
        .filter(|u| u.active)
        .filter_map(|u| {
            if u.score > 10 && u.score % 2 == 1 {
                Some(format!("{}-{}", u.name.to_uppercase(), u.score))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(", ");

    print!("{}", out);
}
