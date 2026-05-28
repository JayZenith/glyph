struct User<'a> {
    name: &'a str,
    score: Option<i32>,
    active: bool,
    bonus: bool,
}

fn build_report(users: &[User], min_score: i32) -> String {
    let picked: Vec<(String, i32)> = users
        .iter()
        .filter_map(|u| {
            let score = u.score?;
            if !u.active || score < min_score {
                return None;
            }
            let final_score = if u.bonus { score } else { score + 1 };
            let label = if u.bonus {
                u.name.to_string()
            } else {
                format!("{}+", u.name)
            };
            Some((label, final_score))
        })
        .collect();

    let total: i32 = picked.iter().map(|(_, score)| *score).sum();
    let entries = picked
        .iter()
        .map(|(label, score)| format!("{}:{}", label, score))
        .collect::<Vec<_>>()
        .join(",");

    format!("selected={} total={} entries={}", picked.len(), total, entries)
}

fn main() {
    let users = [
        User {
            name: "ann",
            score: Some(7),
            active: true,
            bonus: false,
        },
        User {
            name: "bob",
            score: Some(5),
            active: false,
            bonus: true,
        },
        User {
            name: "cam",
            score: Some(9),
            active: true,
            bonus: true,
        },
        User {
            name: "dez",
            score: None,
            active: true,
            bonus: true,
        },
        User {
            name: "eve",
            score: Some(6),
            active: true,
            bonus: false,
        },
    ];

    println!("{}", build_report(&users, 7));
}
