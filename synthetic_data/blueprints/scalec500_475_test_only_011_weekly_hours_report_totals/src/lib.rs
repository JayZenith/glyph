use std::collections::BTreeMap;

pub fn department_report(entries: &[(&str, u32)]) -> String {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();
    for (dept, hours) in entries {
        *totals.entry(*dept).or_insert(0) += *hours;
    }

    let mut lines = Vec::new();
    let mut grand_total = 0;
    for (dept, total) in totals {
        grand_total += total;
        lines.push(format!("{}:{}", dept, total));
    }
    lines.push(format!("total:{}", grand_total));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::department_report;

    #[test]
    fn groups_duplicate_departments_and_sums_total() {
        let entries = [
            ("sales", 5),
            ("ops", 3),
            ("sales", 7),
            ("hr", 2),
            ("ops", 4),
        ];
        let report = department_report(&entries);
        assert_eq!(report, "hr:2\nops:7\nsales:12\ntotal:21");
    }

    #[test]
    fn empty_input_reports_only_zero_total() {
        let report = department_report(&[]);
        assert_eq!(report, "total:0");
    }

    #[test]
    fn single_department_has_one_line_plus_total() {
        let report = department_report(&[("support", 8), ("support", 1)]);
        assert_eq!(report, "support:9\ntotal:9");
    }
}
