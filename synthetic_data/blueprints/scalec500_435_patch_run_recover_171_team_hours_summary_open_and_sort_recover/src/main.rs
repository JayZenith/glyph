use std::collections::BTreeMap;

struct Task {
    team: &'static str,
    hours: u32,
    billable: bool,
    closed: bool,
}

fn main() {
    let tasks = vec![
        Task { team: "ENG", hours: 4, billable: true, closed: false },
        Task { team: "ENG", hours: 1, billable: false, closed: true },
        Task { team: "ENG", hours: 2, billable: true, closed: true },
        Task { team: "OPS", hours: 3, billable: true, closed: false },
        Task { team: "OPS", hours: 3, billable: true, closed: true },
        Task { team: "QA", hours: 2, billable: true, closed: false },
        Task { team: "QA", hours: 5, billable: false, closed: true },
        Task { team: "DOC", hours: 1, billable: false, closed: false },
    ];

    let mut summary: BTreeMap<&str, (u32, u32)> = BTreeMap::new();
    for task in tasks {
        let entry = summary.entry(task.team).or_insert((0, 0));
        entry.0 += task.hours;
        if task.closed {
            entry.1 += 1;
        }
    }

    let mut rows: Vec<_> = summary.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let output = rows
        .into_iter()
        .map(|(team, (hours, open))| format!("{} | {}h | {} open", team, hours, open))
        .collect::<Vec<_>>()
        .join("\n");

    print!("{}", output);
}
