use std::collections::BTreeMap;

pub fn build_report(input: &str) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();
    let mut valid = 0;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let mut parts = line.split(',');
        let category = parts.next().unwrap_or("").trim();
        let amount = parts.next().unwrap_or("").trim().parse::<i32>();
        if category.is_empty() {
            continue;
        }
        if let Ok(value) = amount {
            *totals.entry(category).or_insert(0) += value;
            valid += 1;
        }
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::new();
    for (category, total) in rows {
        out.push_str(&format!("{}:{}\n", category, total));
    }
    out.push_str(&format!("valid={}", valid));
    out
}

#[cfg(test)]
mod tests {
    use super::build_report;

    #[test]
    fn report_sorts_by_total_then_name_and_counts_invalid() {
        let input = "food,10\nbooks,7\nfood,5\ngames,7\nmovies,-2\nbadline\n,3\ntools,abc\n";
        let expected = "food:15\nbooks:7\ngames:7\nvalid=4 invalid=4";
        assert_eq!(build_report(input), expected);
    }

    #[test]
    fn empty_lines_are_ignored_but_invalid_zero_and_missing_fields_count() {
        let input = "\nalpha,2\n\nbeta,0\nalpha,3\ngamma\n";
        let expected = "alpha:5\nvalid=2 invalid=2";
        assert_eq!(build_report(input), expected);
    }
}
