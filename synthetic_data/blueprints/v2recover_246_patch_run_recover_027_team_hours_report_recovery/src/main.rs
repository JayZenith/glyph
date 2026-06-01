fn main() {
    let entries = [
        ("Ann", "done", 2),
        ("Bo", "todo", 4),
        ("Cy", "done", 1),
        ("Bo", "done", 3),
        ("Ann", "todo", 5),
        ("Bo", "done", 2),
    ];

    let mut rows: Vec<(&str, i32, i32)> = Vec::new();
    let mut total_tasks = 0;
    let mut total_hours = 0;

    for (name, status, hours) in entries {
        if status == "done" {
            total_tasks += 1;
            total_hours += hours;

            let mut found = false;
            for row in rows.iter_mut() {
                if row.0 == name {
                    row.1 += 1;
                    found = true;
                    break;
                }
            }
            if !found {
                rows.push((name, 1, 0));
            }
        }
    }

    rows.sort_by(|a, b| b.2.cmp(&a.2));

    println!("All Hours Report");
    for (name, tasks, hours) in rows {
        println!("{}: {} tasks, {}h", name, tasks, hours);
    }
    println!("TOTAL: {} tasks, {}h", total_tasks, total_hours);
}
