struct User {
    name: &'static str,
    active: bool,
    score: Option<i32>,
}

fn main() {
    let users = vec![
        User { name: "Ada", active: true, score: Some(5) },
        User { name: "Bea", active: false, score: Some(7) },
        User { name: "Cid", active: true, score: None },
        User { name: "Dee", active: true, score: Some(0) },
        User { name: "Eli", active: true, score: Some(6) },
    ];

    let lines: Vec<String> = users
        .iter()
        .filter_map(|u| {
            if u.active && u.score.unwrap_or(0) > 0 {
                Some(format!("{}:{}", u.name, u.score.unwrap_or(0)))
            } else {
                None
            }
        })
        .collect();

    let total: i32 = users
        .iter()
        .filter(|u| u.active)
        .map(|u| u.score.unwrap_or(0))
        .sum();

    for line in lines {
        println!("{}", line);
    }
    println!("TOTAL:{}", total);
}
