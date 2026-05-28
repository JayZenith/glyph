use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub team: &'static str,
    pub hours: u32,
    pub billable: bool,
}

pub fn team_billable_hours(entries: &[Entry]) -> Vec<(String, u32)> {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for entry in entries {
        if entry.billable {
            *totals.entry(entry.team).or_insert(0) += entry.hours;
        }
    }

    totals
        .into_iter()
        .map(|(team, hours)| (team.to_string(), hours))
        .collect()
}

pub fn report_lines(entries: &[Entry]) -> Vec<String> {
    team_billable_hours(entries)
        .into_iter()
        .map(|(team, hours)| format!("{}: {}h", team, hours))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_only_billable_hours_per_team() {
        let entries = vec![
            Entry { team: "alpha", hours: 3, billable: true },
            Entry { team: "beta", hours: 5, billable: false },
            Entry { team: "alpha", hours: 2, billable: true },
            Entry { team: "beta", hours: 4, billable: true },
        ];

        let totals = team_billable_hours(&entries);
        assert_eq!(totals, vec![
            ("alpha".to_string(), 5),
            ("beta".to_string(), 4),
        ]);
    }

    #[test]
    fn report_is_sorted_by_team_name_and_skips_zero_teams() {
        let entries = vec![
            Entry { team: "zeta", hours: 1, billable: true },
            Entry { team: "alpha", hours: 2, billable: true },
            Entry { team: "zeta", hours: 3, billable: true },
            Entry { team: "gamma", hours: 9, billable: false },
        ];

        let lines = report_lines(&entries);
        assert_eq!(lines, vec![
            "alpha: 2h".to_string(),
            "zeta: 4h".to_string(),
        ]);
    }

    #[test]
    fn empty_when_no_billable_entries() {
        let entries = vec![
            Entry { team: "alpha", hours: 7, billable: false },
            Entry { team: "beta", hours: 1, billable: false },
        ];

        assert!(team_billable_hours(&entries).is_empty());
        assert!(report_lines(&entries).is_empty());
    }
}
