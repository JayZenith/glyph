use std::collections::BTreeMap;

pub fn department_totals(entries: &[(&str, u32)]) -> Vec<String> {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();
    for (dept, amount) in entries {
        *totals.entry(*dept).or_insert(0) += *amount;
    }

    totals
        .into_iter()
        .map(|(dept, total)| format!("{}:{}", dept, total))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::department_totals;

    #[test]
    fn groups_and_sums_by_department() {
        let entries = [
            ("ops", 3),
            ("sales", 5),
            ("ops", 4),
            ("sales", 2),
            ("hr", 1),
        ];
        assert_eq!(
            department_totals(&entries),
            vec!["hr:1", "ops:7", "sales:7"]
        );
    }

    #[test]
    fn empty_input_yields_empty_report() {
        let entries: [(&str, u32); 0] = [];
        let report = department_totals(&entries);
        assert!(report.is_empty());
    }

    #[test]
    fn preserves_sorted_department_order() {
        let entries = [("zeta", 1), ("alpha", 2), ("mu", 4)];
        assert_eq!(department_totals(&entries), vec!["alpha:2", "mu:4", "zeta:1"]);
    }
}
