use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale<'a> {
    pub region: &'a str,
    pub amount: i32,
}

pub fn sales_report(rows: &[Sale<'_>]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for row in rows {
        *totals.entry(row.region).or_insert(0) += row.amount;
    }

    let mut items: Vec<(&str, i32)> = totals.into_iter().collect();
    items.sort_by(|a, b| a.0.cmp(b.0));

    items
        .into_iter()
        .map(|(region, total)| format!("{}:{}", region, total))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregates_positive_amounts_only_and_sorts_by_total_then_name() {
        let rows = [
            Sale { region: "west", amount: 5 },
            Sale { region: "east", amount: 7 },
            Sale { region: "west", amount: -2 },
            Sale { region: "north", amount: 7 },
            Sale { region: "east", amount: 0 },
            Sale { region: "south", amount: 1 },
        ];

        assert_eq!(sales_report(&rows), "east:7\nnorth:7\nwest:5\nsouth:1");
    }

    #[test]
    fn omits_regions_with_zero_final_total_after_filtering() {
        let rows = [
            Sale { region: "east", amount: -3 },
            Sale { region: "west", amount: 0 },
            Sale { region: "north", amount: 4 },
        ];

        assert_eq!(sales_report(&rows), "north:4");
    }
}
