use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Record {
    team: &'static str,
    points: u32,
    active: bool,
}

fn main() {
    let records = vec![
        Record { team: "alpha", points: 5, active: true },
        Record { team: "alpha", points: 7, active: true },
        Record { team: "beta", points: 10, active: true },
        Record { team: "beta", points: 1, active: true },
        Record { team: "gamma", points: 4, active: true },
        Record { team: "gamma", points: 4, active: true },
        Record { team: "beta", points: 99, active: false },
    ];

    let mut stats: BTreeMap<&str, (u32, u32)> = BTreeMap::new();
    for r in records {
        let entry = stats.entry(r.team).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += r.points;
    }

    let mut lines = vec![String::from("Team Summary")];
    let mut high = Vec::new();
    for (team, (count, total)) in stats {
        let avg = total as f64 / count as f64;
        if avg >= 6.0 {
            high.push(team.to_string());
        }
        lines.push(format!("{}: count={} total={} avg={:.1}", team, count, total, avg));
    }
    lines.push(format!("High performers: {}", high.join(",")));

    println!("{}", lines.join("\n"));
}
