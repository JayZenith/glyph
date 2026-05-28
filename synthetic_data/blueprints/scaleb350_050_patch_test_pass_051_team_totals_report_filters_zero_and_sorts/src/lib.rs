use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub team: String,
    pub completed: u32,
}

pub fn build_report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<String, u32> = BTreeMap::new();
    for entry in entries {
        *totals.entry(entry.team.clone()).or_insert(0) += entry.completed;
    }

    let mut lines: Vec<String> = totals
        .into_iter()
        .map(|(team, total)| format!("{} ({})", team, total))
        .collect();
    lines.sort();
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{build_report, Entry};

    fn e(team: &str, completed: u32) -> Entry {
        Entry {
            team: team.to_string(),
            completed,
        }
    }

    #[test]
    fn aggregates_sorts_and_omits_zero_totals() {
        let entries = vec![
            e("ops", 2),
            e("design", 4),
            e("ops", 3),
            e("qa", 0),
            e("design", 1),
            e("support", 5),
        ];

        assert_eq!(
            build_report(&entries),
            "design: 5 tasks\nops: 5 tasks\nsupport: 5 tasks"
        );
    }

    #[test]
    fn empty_after_filtering_returns_empty_string() {
        let entries = vec![e("ops", 0), e("qa", 0)];
        assert_eq!(build_report(&entries), "");
    }

    #[test]
    fn ties_are_broken_by_team_name() {
        let entries = vec![e("zeta", 2), e("alpha", 2), e("beta", 3)];
        assert_eq!(
            build_report(&entries),
            "beta: 3 tasks\nalpha: 2 tasks\nzeta: 2 tasks"
        );
    }
}
