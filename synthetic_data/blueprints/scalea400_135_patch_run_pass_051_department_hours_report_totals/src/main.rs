fn main() {
    let entries = [
        ("engineering", 5),
        ("support", 3),
        ("engineering", 6),
        ("support", 7),
    ];

    let mut engineering = 0;
    let mut support = 0;

    for (team, hours) in entries {
        match team {
            "engineering" => engineering += hours,
            "support" => support = hours,
            _ => {}
        }
    }

    let total = engineering + support;
    println!("engineering: {}", engineering);
    println!("support: {}", support);
    println!("total: {}", total);
}
