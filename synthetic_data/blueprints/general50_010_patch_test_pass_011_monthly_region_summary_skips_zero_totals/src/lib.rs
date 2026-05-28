use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub month: &'static str,
    pub region: &'static str,
    pub approved: bool,
    pub amount: i32,
}

pub fn summarize(records: &[Record]) -> String {
    let mut by_month: BTreeMap<&str, BTreeMap<&str, i32>> = BTreeMap::new();

    for record in records {
        let month_entry = by_month.entry(record.month).or_default();
        let value = month_entry.entry(record.region).or_insert(0);
        if record.approved {
            *value += record.amount;
        }
    }

    let mut out = String::new();
    for (month, regions) in by_month {
        out.push_str(month);
        out.push('\n');
        for (region, total) in regions {
            out.push_str("  ");
            out.push_str(region);
            out.push_str(": ");
            out.push_str(&total.to_string());
            out.push('\n');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{summarize, Record};

    #[test]
    fn groups_by_month_and_region_and_uses_sorted_output() {
        let records = vec![
            Record { month: "2024-01", region: "west", approved: true, amount: 7 },
            Record { month: "2024-01", region: "east", approved: true, amount: 3 },
            Record { month: "2024-01", region: "west", approved: true, amount: 2 },
            Record { month: "2024-02", region: "north", approved: true, amount: 5 },
        ];

        let expected = "2024-01\n  east: 3\n  west: 9\n2024-02\n  north: 5\n";
        assert_eq!(summarize(&records), expected);
    }

    #[test]
    fn ignores_unapproved_records_and_omits_zero_total_regions() {
        let records = vec![
            Record { month: "2024-03", region: "east", approved: false, amount: 50 },
            Record { month: "2024-03", region: "west", approved: true, amount: 4 },
            Record { month: "2024-03", region: "west", approved: false, amount: 100 },
            Record { month: "2024-03", region: "north", approved: true, amount: 0 },
        ];

        let expected = "2024-03\n  west: 4\n";
        assert_eq!(summarize(&records), expected);
    }
}
