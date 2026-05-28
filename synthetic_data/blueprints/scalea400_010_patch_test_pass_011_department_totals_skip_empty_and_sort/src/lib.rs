use std::collections::BTreeMap;

pub fn summarize_hours(entries: &[(&str, u32)]) -> String {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for (dept, hours) in entries {
        *totals.entry(dept).or_insert(0) += *hours;
    }

    let mut lines = Vec::new();
    for (dept, total) in totals {
        lines.push(format!("{}:{}", dept, total));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::summarize_hours;

    #[test]
    fn groups_filters_and_sorts_report_lines() {
        let entries = [
            ("ops", 2),
            ("sales", 0),
            ("", 5),
            ("ops", 3),
            ("hr", 5),
            ("legal", 1),
            ("sales", 4),
        ];

        assert_eq!(summarize_hours(&entries), "hr:5\nops:5\nsales:4\nlegal:1");
    }

    #[test]
    fn returns_empty_string_when_nothing_reportable() {
        let entries = [("", 2), ("sales", 0), ("", 0)];
        assert_eq!(summarize_hours(&entries), "");
    }
}
