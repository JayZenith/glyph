use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub team: &'static str,
    pub hours: u32,
    pub billable: bool,
}

pub fn render_team_report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for entry in entries {
        let slot = totals.entry(entry.team).or_insert((0, 0));
        slot.0 += entry.hours;
        if entry.billable {
            slot.1 += entry.hours;
        }
    }

    let mut lines = Vec::new();
    for (team, (total, billable)) in totals {
        if total == 0 {
            continue;
        }
        let pct = if total == 0 { 0 } else { billable * 100 / total };
        lines.push(format!("{}: total={} billable={} rate={}%,", team, total, billable, pct));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_sorted_teams_with_percentages() {
        let entries = [
            Entry { team: "beta", hours: 2, billable: true },
            Entry { team: "alpha", hours: 3, billable: false },
            Entry { team: "beta", hours: 3, billable: false },
            Entry { team: "alpha", hours: 1, billable: true },
        ];

        let report = render_team_report(&entries);
        assert_eq!(report, "alpha: total=4 billable=1 rate=25%\nbeta: total=5 billable=2 rate=40%");
    }

    #[test]
    fn skips_zero_total_teams_and_returns_fallback_when_empty() {
        let entries = [
            Entry { team: "ops", hours: 0, billable: true },
            Entry { team: "dev", hours: 0, billable: false },
        ];

        assert_eq!(render_team_report(&entries), "no teams");
        assert_eq!(render_team_report(&[]), "no teams");
    }

    #[test]
    fn billable_hours_are_capped_by_total_hours() {
        let entries = [
            Entry { team: "qa", hours: 0, billable: true },
            Entry { team: "qa", hours: 2, billable: false },
        ];

        assert_eq!(render_team_report(&entries), "qa: total=2 billable=0 rate=0%");
    }
}
