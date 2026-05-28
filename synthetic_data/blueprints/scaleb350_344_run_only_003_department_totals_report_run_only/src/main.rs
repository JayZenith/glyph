fn main() {
    let records = [
        ("Engineering", 3),
        ("Sales", 5),
        ("Engineering", 4),
        ("Marketing", 4),
        ("Sales", 3),
        ("Engineering", 2),
    ];

    let departments = ["Engineering", "Marketing", "Sales"];
    let mut grand_total = 0;

    for dept in departments {
        let dept_total: i32 = records
            .iter()
            .filter(|(name, _)| *name == dept)
            .map(|(_, count)| *count)
            .sum();
        grand_total += dept_total;
        println!("{}: {} tasks", dept, dept_total);
    }

    println!("Grand total: {} tasks", grand_total);
}
