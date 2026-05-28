use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub team: &'static str,
    pub hours: u32,
    pub active: bool,
}

pub fn build_report(entries: &[Entry]) -> Vec<String> {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for entry in entries {
        *totals.entry(entry.team).or_insert(0) += entry.hours;
    }

    let mut rows: Vec<(&str, u32)> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(team, hours)| format!("{team}: {hours}h"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_active_hours_and_sorts_by_total_then_name() {
        let entries = vec![
            Entry { team: "beta", hours: 3, active: true },
            Entry { team: "alpha", hours: 5, active: true },
            Entry { team: "beta", hours: 4, active: true },
            Entry { team: "gamma", hours: 7, active: false },
            Entry { team: "alpha", hours: 4, active: true },
        ];

        assert_eq!(
            build_report(&entries),
            vec![
                "alpha: 9h".to_string(),
                "beta: 7h".to_string(),
            ]
        );
    }

    #[test]
    fn skips_zero_total_teams_after_filtering_inactive_entries() {
        let entries = vec![
            Entry { team: "ops", hours: 0, active: true },
            Entry { team: "sales", hours: 2, active: false },
            Entry { team: "sales", hours: 0, active: true },
            Entry { team: "dev", hours: 1, active: true },
        ];

        assert_eq!(
            build_report(&entries),
            vec!["dev: 1h".to_string()]
        );
    }

    #[test]
    fn uses_team_name_as_tiebreak_for_equal_totals() {
        let entries = vec![
            Entry { team: "zeta", hours: 2, active: true },
            Entry { team: "eta", hours: 2, active: true },
            Entry { team: "zeta", hours: 1, active: true },
            Entry { team: "eta", hours: 1, active: true },
        ];

        assert_eq!(
            build_report(&entries),
            vec![
                "eta: 3h".to_string(),
                "zeta: 3h".to_string(),
            ]
        );
    }
}
