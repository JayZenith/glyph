use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Task {
    team: &'static str,
    hours: u32,
    done: bool,
}

fn main() {
    let tasks = [
        Task { team: "alpha", hours: 3, done: true },
        Task { team: "beta", hours: 2, done: false },
        Task { team: "alpha", hours: 4, done: false },
        Task { team: "gamma", hours: 5, done: true },
        Task { team: "beta", hours: 5, done: true },
    ];

    let mut summary: BTreeMap<&str, (u32, u32)> = BTreeMap::new();
    let mut open_tasks = 0u32;

    for task in tasks {
        let entry = summary.entry(task.team).or_insert((0, 0));
        entry.0 += 1;
        if !task.done {
            entry.1 += task.hours;
            open_tasks += 1;
        }
    }

    println!("Team Summary");
    for (team, (count, hours)) in summary {
        println!("{}: tasks={} hours={}", team, count, hours);
    }
    println!("Open tasks total: {}", open_tasks);
}
