use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event<'a> {
    pub team: &'a str,
    pub points: u32,
    pub billable: bool,
}

pub fn summarize(events: &[Event<'_>]) -> String {
    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for e in events {
        let entry = totals.entry(e.team).or_insert((0, 0));
        entry.0 += e.points;
        entry.1 += 1;
    }

    let mut lines = Vec::new();
    for (team, (points, count)) in totals {
        if points == 0 {
            continue;
        }
        lines.push(format!("{}: {} pts ({} items)", team, points, count));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{summarize, Event};

    #[test]
    fn groups_sorted_and_ignores_zero_billable_totals() {
        let events = [
            Event { team: "beta", points: 5, billable: true },
            Event { team: "alpha", points: 0, billable: true },
            Event { team: "alpha", points: 7, billable: true },
            Event { team: "alpha", points: 4, billable: false },
            Event { team: "gamma", points: 3, billable: false },
        ];

        assert_eq!(summarize(&events), "alpha: 7 pts (1 items)\nbeta: 5 pts (1 items)");
    }

    #[test]
    fn keeps_empty_report_when_no_billable_points_exist() {
        let events = [
            Event { team: "ops", points: 0, billable: true },
            Event { team: "ops", points: 6, billable: false },
            Event { team: "sales", points: 0, billable: false },
        ];

        assert_eq!(summarize(&events), "");
    }

    #[test]
    fn counts_only_billable_items_even_when_zero_point_billable_exists() {
        let events = [
            Event { team: "red", points: 0, billable: true },
            Event { team: "red", points: 2, billable: true },
            Event { team: "red", points: 8, billable: true },
            Event { team: "red", points: 9, billable: false },
        ];

        assert_eq!(summarize(&events), "red: 10 pts (2 items)");
    }
}
