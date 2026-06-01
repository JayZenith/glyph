struct User {
    name: &'static str,
    score: i32,
    active: bool,
}

fn active_score_labels(users: &[User]) -> String {
    users
        .iter()
        .filter(|u| u.score >= 5)
        .map(|u| format!("{}:{}", u.name, u.score))
        .collect::<Vec<_>>()
        .join(", ")
}

fn main() {
    let users = [
        User {
            name: "Ava",
            score: 8,
            active: true,
        },
        User {
            name: "Ben",
            score: 4,
            active: true,
        },
        User {
            name: "Cleo",
            score: 5,
            active: true,
        },
        User {
            name: "Drew",
            score: 9,
            active: true,
        },
        User {
            name: "Eli",
            score: 7,
            active: false,
        },
    ];

    println!("{}", active_score_labels(&users));
}
