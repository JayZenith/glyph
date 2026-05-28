use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub team: &'static str,
    pub points: i32,
    pub active: bool,
}

pub fn render_team_report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for entry in entries {
        if entry.active {
            *totals.entry(entry.team).or_insert(0) += entry.points;
        }
    }

    totals
        .into_iter()
        .map(|(team, total)| format!("{team}: {total}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_sorted_totals_for_active_entries() {
        let entries = [
            Entry { team: "beta", points: 2, active: true },
            Entry { team: "alpha", points: 5, active: true },
            Entry { team: "beta", points: 4, active: true },
            Entry { team: "alpha", points: -2, active: false },
        ];

        assert_eq!(render_team_report(&entries), "alpha: 5\nbeta: 6");
    }

    #[test]
    fn omits_teams_whose_active_total_is_zero() {
        let entries = [
            Entry { team: "ops", points: 3, active: true },
            Entry { team: "ops", points: -3, active: true },
            Entry { team: "sales", points: 2, active: true },
            Entry { team: "sales", points: 1, active: false },
        ];

        assert_eq!(render_team_report(&entries), "sales: 2");
    }

    #[test]
    fn returns_empty_string_when_no_active_totals_remain() {
        let entries = [
            Entry { team: "ops", points: 0, active: true },
            Entry { team: "ops", points: 1, active: false },
            Entry { team: "qa", points: 7, active: false },
        ];

        assert_eq!(render_team_report(&entries), "");
    }
}
