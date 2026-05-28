fn main() {
    let entries = [
        ("ops", 3),
        ("design", 5),
        ("ops", 4),
        ("sales", 2),
        ("design", 3),
        ("sales", 3),
    ];

    let departments = ["ops", "design", "sales"];
    let mut lines = Vec::new();
    let mut grand_total = 0;

    for dept in departments {
        let subtotal: i32 = entries
            .iter()
            .filter(|(name, _)| *name == dept)
            .map(|(_, hours)| *hours)
            .sum();
        grand_total += subtotal;
        lines.push(format!("{}: {}", dept, subtotal));
    }

    lines.push(format!("total: {}", grand_total));
    print!("{}", lines.join("\n"));
}
