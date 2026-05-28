use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub region: &'static str,
    pub item: &'static str,
    pub count: u32,
}

pub fn build_report(sales: &[Sale]) -> String {
    let mut grouped: BTreeMap<&str, Vec<&Sale>> = BTreeMap::new();

    for sale in sales {
        grouped.entry(sale.region).or_default().push(sale);
    }

    let mut lines = Vec::new();

    for (region, entries) in grouped {
        let total: u32 = entries.iter().map(|s| s.count).sum();
        let mut parts: Vec<String> = entries
            .iter()
            .map(|s| format!("{}:{}", s.item, s.count))
            .collect();
        parts.sort();
        lines.push(format!("{} total={} [{}]", region, total, parts.join(", ")));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_zero_count_entries_in_totals_and_details() {
        let sales = [
            Sale { region: "east", item: "apples", count: 3 },
            Sale { region: "east", item: "bananas", count: 0 },
            Sale { region: "west", item: "dates", count: 0 },
            Sale { region: "west", item: "carrots", count: 5 },
        ];

        let report = build_report(&sales);
        assert_eq!(
            report,
            "east total=3 [apples:3]\nwest total=5 [carrots:5]"
        );
    }

    #[test]
    fn keeps_regions_sorted_and_item_details_sorted() {
        let sales = [
            Sale { region: "south", item: "oranges", count: 2 },
            Sale { region: "north", item: "beets", count: 4 },
            Sale { region: "north", item: "artichokes", count: 1 },
        ];

        let report = build_report(&sales);
        assert_eq!(
            report,
            "north total=5 [artichokes:1, beets:4]\nsouth total=2 [oranges:2]"
        );
    }
}
