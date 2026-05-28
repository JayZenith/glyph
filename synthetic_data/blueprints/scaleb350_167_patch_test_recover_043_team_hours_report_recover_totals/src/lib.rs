use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Entry {
    pub team: &'static str,
    pub hours: u32,
    pub approved: bool,
}

pub fn build_report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();
    let mut grand_total = 0;

    for entry in entries {
        *totals.entry(entry.team).or_insert(0) += entry.hours;
        grand_total += entry.hours;
    }

    let mut lines = vec!["Team Hours Report".to_string()];
    for (team, hours) in totals {
        lines.push(format!("{team}: {hours}h"));
    }
    lines.push(format!("Grand total: {grand_total}h approved"));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{build_report, Entry};

    #[test]
    fn filters_rejected_and_sorts_by_total_desc() {
        let entries = [
            Entry { team: "Ops", hours: 2, approved: true },
            Entry { team: "QA", hours: 3, approved: true },
            Entry { team: "Ops", hours: 4, approved: false },
            Entry { team: "Platform", hours: 5, approved: true },
            Entry { team: "QA", hours: 1, approved: true },
        ];

        let report = build_report(&entries);
        let expected = "Team Hours Report\nPlatform: 5h\nQA: 4h\nOps: 2h\nGrand total approved: 11h";
        assert_eq!(report, expected);
    }

    #[test]
    fn tie_breaks_by_team_name() {
        let entries = [
            Entry { team: "Delta", hours: 4, approved: true },
            Entry { team: "Alpha", hours: 4, approved: true },
            Entry { team: "Beta", hours: 1, approved: false },
        ];

        let report = build_report(&entries);
        let expected = "Team Hours Report\nAlpha: 4h\nDelta: 4h\nGrand total approved: 8h";
        assert_eq!(report, expected);
    }
}
