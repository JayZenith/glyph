struct User {
    name: &'static str,
    active: bool,
    score: Option<i32>,
    bonus: i32,
}

fn build_report(users: &[User]) -> String {
    let rows: Vec<String> = users
        .iter()
        .filter(|u| u.active)
        .filter_map(|u| {
            u.score.and_then(|score| {
                let total = score + u.bonus;
                (total > 0).then(|| format!("{}={}", u.name, total))
            })
        })
        .collect();

    let total: i32 = users
        .iter()
        .filter(|u| u.active)
        .filter_map(|u| u.score.map(|score| score + u.bonus))
        .filter(|n| *n > 0)
        .sum();

    format!("{}\nTOTAL {}", rows.join(", "), total)
}

fn main() {
    let users = [
        User {
            name: "Ava",
            active: true,
            score: Some(7),
            bonus: 3,
        },
        User {
            name: "Bo",
            active: false,
            score: Some(9),
            bonus: 2,
        },
        User {
            name: "Cy",
            active: true,
            score: Some(10),
            bonus: -2,
        },
        User {
            name: "Dee",
            active: true,
            score: None,
            bonus: 0,
        },
        User {
            name: "Eli",
            active: true,
            score: Some(1),
            bonus: -5,
        },
    ];

    println!("{}", build_report(&users));
}
