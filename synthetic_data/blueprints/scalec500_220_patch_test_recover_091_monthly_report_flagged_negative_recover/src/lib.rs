use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub category: &'static str,
    pub amount: i32,
    pub valid: bool,
}

pub fn build_report(entries: &[Entry]) -> String {
    let mut groups: BTreeMap<&str, (i32, usize, bool)> = BTreeMap::new();

    for e in entries {
        let bucket = groups.entry(e.category).or_insert((0, 0, false));
        bucket.0 += e.amount;
        if e.valid {
            bucket.1 += 1;
        }
    }

    let mut lines = Vec::new();
    for (category, (sum, valid_count, has_negative)) in groups {
        let mut line = format!("{}: total={}, valid={}", category, sum, valid_count);
        if has_negative {
            line.push_str(" [check]");
        }
        lines.push(line);
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_and_counts_only_valid_entries() {
        let entries = [
            Entry { category: "food", amount: 12, valid: true },
            Entry { category: "food", amount: 8, valid: false },
            Entry { category: "travel", amount: 30, valid: true },
            Entry { category: "travel", amount: -5, valid: true },
        ];

        let got = build_report(&entries);
        let want = "food: total=12, valid=1\ntravel: total=25, valid=2 [check]";
        assert_eq!(got, want);
    }

    #[test]
    fn sorts_categories_and_marks_negative_even_when_invalid() {
        let entries = [
            Entry { category: "zeta", amount: 4, valid: true },
            Entry { category: "alpha", amount: 10, valid: true },
            Entry { category: "alpha", amount: -3, valid: false },
        ];

        let got = build_report(&entries);
        let want = "alpha: total=10, valid=1 [check]\nzeta: total=4, valid=1";
        assert_eq!(got, want);
    }
}
