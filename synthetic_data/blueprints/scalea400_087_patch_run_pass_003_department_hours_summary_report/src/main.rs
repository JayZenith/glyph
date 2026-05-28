use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Task {
    dept: &'static str,
    hours: u32,
    billable: bool,
}

fn report(tasks: &[Task]) -> String {
    let mut by_dept: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for task in tasks {
        if !task.billable {
            continue;
        }
        let entry = by_dept.entry(task.dept).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += task.hours;
    }

    let mut lines = Vec::new();
    let mut total = 0;

    for (dept, (hours, count)) in by_dept {
        total += hours;
        lines.push(format!("{}: {}h ({} tasks)", dept, hours, count));
    }

    lines.push(format!("TOTAL: {}h", total));
    lines.join("\n")
}

fn main() {
    let tasks = [
        Task { dept: "ENG", hours: 5, billable: true },
        Task { dept: "ENG", hours: 3, billable: true },
        Task { dept: "ENG", hours: 2, billable: false },
        Task { dept: "OPS", hours: 7, billable: true },
        Task { dept: "OPS", hours: 1, billable: true },
        Task { dept: "SALES", hours: 4, billable: true },
        Task { dept: "SALES", hours: 2, billable: true },
        Task { dept: "SALES", hours: 3, billable: false },
    ];

    println!("{}", report(&tasks));
}
