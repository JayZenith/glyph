use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub category: &'static str,
    pub minutes: u32,
    pub billable: bool,
}

pub fn weekly_report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for entry in entries {
        let bucket = totals.entry(entry.category).or_insert((0, 0));
        bucket.0 += 1;
        if entry.billable {
            bucket.1 += entry.minutes;
        }
    }

    let mut lines = vec!["Weekly Report".to_string()];
    let mut grand_entries = 0u32;
    let mut grand_minutes = 0u32;

    for (category, (count, minutes)) in totals {
        grand_entries += count;
        grand_minutes += minutes;
        lines.push(format!("{}: {} items, {} min", category, count, minutes));
    }

    lines.push(format!("TOTAL: {} items, {} min", grand_entries, grand_minutes));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_and_sorts_categories_and_counts_all_minutes() {
        let entries = [
            Entry { category: "ops", minutes: 30, billable: true },
            Entry { category: "dev", minutes: 45, billable: false },
            Entry { category: "ops", minutes: 15, billable: true },
            Entry { category: "dev", minutes: 60, billable: true },
        ];

        let report = weekly_report(&entries);
        let expected = "Weekly Report\ndev: 2 entries, 105 min\nops: 2 entries, 45 min\nTOTAL: 4 entries, 150 min";
        assert_eq!(report, expected);
    }

    #[test]
    fn skips_zero_minute_categories_but_keeps_total_entry_count() {
        let entries = [
            Entry { category: "support", minutes: 0, billable: false },
            Entry { category: "research", minutes: 20, billable: false },
            Entry { category: "support", minutes: 0, billable: true },
        ];

        let report = weekly_report(&entries);
        let expected = "Weekly Report\nresearch: 1 entry, 20 min\nTOTAL: 3 entries, 20 min";
        assert_eq!(report, expected);
    }
}
