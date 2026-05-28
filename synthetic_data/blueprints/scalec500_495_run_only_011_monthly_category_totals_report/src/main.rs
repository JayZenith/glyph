fn main() {
    let entries = [
        ("March", "food", 12),
        ("April", "food", 5),
        ("March", "travel", 30),
        ("March", "food", 5),
        ("March", "utilities", 45),
    ];

    let mut food = 0;
    let mut travel = 0;
    let mut utilities = 0;

    for (month, category, amount) in entries {
        if month != "March" {
            continue;
        }
        match category {
            "food" => food += amount,
            "travel" => travel += amount,
            "utilities" => utilities += amount,
            _ => {}
        }
    }

    let grand_total = food + travel + utilities;

    println!("March totals:");
    println!("food: {}", food);
    println!("travel: {}", travel);
    println!("utilities: {}", utilities);
    println!("grand total: {}", grand_total);
}
