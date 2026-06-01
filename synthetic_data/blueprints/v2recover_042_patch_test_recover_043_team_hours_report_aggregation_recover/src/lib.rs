use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Entry {
    pub team: &'static str,
    pub hours: f64,
    pub approved: bool,
}

pub fn build_report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, f64> = BTreeMap::new();

    for entry in entries {
        let bucket = totals.entry(entry.team).or_insert(0.0);
        *bucket += entry.hours;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows
        .into_iter()
        .map(|(team, hours)| format!("{}: {:.1}h", team, hours))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{build_report, Entry};

    #[test]
    fn groups_approved_hours_and_orders_report() {
        let entries = [
            Entry { team: "Ops", hours: 1.5, approved: true },
            Entry { team: "App", hours: 2.0, approved: true },
            Entry { team: "Ops", hours: 0.5, approved: true },
            Entry { team: "Biz", hours: 4.0, approved: false },
            Entry { team: "App", hours: 3.0, approved: true },
            Entry { team: "Zero", hours: 0.0, approved: true },
        ];

        assert_eq!(build_report(&entries), "App: 5h\nOps: 2h");
    }

    #[test]
    fn tie_breaks_same_totals_by_team_name() {
        let entries = [
            Entry { team: "Zulu", hours: 1.0, approved: true },
            Entry { team: "Alpha", hours: 1.0, approved: true },
            Entry { team: "Zulu", hours: 2.0, approved: true },
            Entry { team: "Alpha", hours: 2.0, approved: true },
            Entry { team: "Hold", hours: 10.0, approved: false },
        ];

        assert_eq!(build_report(&entries), "Alpha: 3h\nZulu: 3h");
    }

    #[test]
    fn keeps_fractional_hours_with_single_decimal() {
        let entries = [
            Entry { team: "Core", hours: 1.2, approved: true },
            Entry { team: "Core", hours: 0.3, approved: true },
            Entry { team: "Edge", hours: 1.4, approved: true },
        ];

        assert_eq!(build_report(&entries), "Core: 1.5h\nEdge: 1.4h");
    }
}
