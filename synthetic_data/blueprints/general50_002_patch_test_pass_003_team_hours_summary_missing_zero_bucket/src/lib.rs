use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub team: &'static str,
    pub hours: u32,
    pub billable: bool,
}

pub fn build_report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for entry in entries {
        if entry.hours == 0 {
            continue;
        }
        let bucket = totals.entry(entry.team).or_insert((0, 0));
        bucket.0 += entry.hours;
        if entry.billable {
            bucket.1 += entry.hours;
        }
    }

    let mut lines = Vec::new();
    for (team, (total, billable)) in totals {
        lines.push(format!("{}: total={} billable={}", team, total, billable));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_zero_hour_team_with_zero_totals() {
        let entries = [
            Entry { team: "alpha", hours: 0, billable: true },
            Entry { team: "beta", hours: 3, billable: true },
        ];

        assert_eq!(
            build_report(&entries),
            "alpha: total=0 billable=0\nbeta: total=3 billable=3"
        );
    }

    #[test]
    fn zero_hour_entries_do_not_affect_billable_math() {
        let entries = [
            Entry { team: "ops", hours: 2, billable: true },
            Entry { team: "ops", hours: 0, billable: true },
            Entry { team: "ops", hours: 5, billable: false },
        ];

        assert_eq!(build_report(&entries), "ops: total=7 billable=2");
    }

    #[test]
    fn report_is_sorted_by_team_name() {
        let entries = [
            Entry { team: "zeta", hours: 1, billable: false },
            Entry { team: "alpha", hours: 4, billable: true },
        ];

        assert_eq!(
            build_report(&entries),
            "alpha: total=4 billable=4\nzeta: total=1 billable=0"
        );
    }
}
