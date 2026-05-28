use std::collections::BTreeMap;

pub fn render_status_report(rows: &[(&str, u32)]) -> String {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();
    for (status, count) in rows {
        *totals.entry(*status).or_insert(0) += *count;
    }

    let mut out = Vec::new();
    for (status, total) in totals {
        out.push(format!("{}:{}", status, total));
    }
    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::render_status_report;

    #[test]
    fn groups_sorts_and_skips_zero_totals() {
        let rows = [
            ("warn", 2),
            ("ok", 0),
            ("fail", 3),
            ("warn", 1),
            ("ok", 2),
            ("idle", 0),
        ];
        assert_eq!(render_status_report(&rows), "fail:3\nwarn:3\nok:2");
    }

    #[test]
    fn alphabetical_tiebreak_for_equal_totals() {
        let rows = [("beta", 1), ("alpha", 1), ("beta", 1), ("alpha", 1)];
        assert_eq!(render_status_report(&rows), "alpha:2\nbeta:2");
    }

    #[test]
    fn empty_when_everything_sums_to_zero() {
        let rows = [("ok", 0), ("warn", 0), ("ok", 0)];
        assert_eq!(render_status_report(&rows), "");
    }
}
