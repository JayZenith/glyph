use std::collections::BTreeMap;

pub fn summarize_hours(entries: &[(&str, u32)]) -> String {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for (team, hours) in entries {
        *totals.entry(team).or_insert(0) += *hours;
    }

    let mut lines = Vec::new();
    for (team, total) in totals {
        if total > 0 {
            lines.push(format!("{}: {}h", team, total));
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::summarize_hours;

    #[test]
    fn groups_and_sorts_departments() {
        let entries = [
            ("support", 2),
            ("engineering", 5),
            ("support", 3),
            ("design", 4),
        ];

        assert_eq!(
            summarize_hours(&entries),
            "design: 4h\nengineering: 5h\nsupport: 5h"
        );
    }

    #[test]
    fn keeps_zero_total_departments() {
        let entries = [("ops", 0), ("sales", 3)];
        assert_eq!(summarize_hours(&entries), "ops: 0h\nsales: 3h");
    }

    #[test]
    fn empty_input_produces_empty_report() {
        assert_eq!(summarize_hours(&[]), "");
    }
}
