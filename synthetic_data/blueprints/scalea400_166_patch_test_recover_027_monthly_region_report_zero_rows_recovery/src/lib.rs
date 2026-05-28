#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub region: &'static str,
    pub amount: i32,
}

pub fn render_region_report(sales: &[Sale]) -> String {
    let mut totals: Vec<(&'static str, i32)> = Vec::new();

    for sale in sales {
        if let Some((_, total)) = totals.iter_mut().find(|(region, _)| *region == sale.region) {
            *total += sale.amount;
        } else {
            totals.push((sale.region, sale.amount));
        }
    }

    totals.retain(|(_, total)| *total > 0);
    totals.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::new();
    for (region, total) in totals {
        out.push_str(region);
        out.push(':');
        out.push(' ');
        out.push_str(&total.to_string());
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{render_region_report, Sale};

    #[test]
    fn sorts_by_total_desc_then_region_name() {
        let sales = [
            Sale { region: "west", amount: 4 },
            Sale { region: "east", amount: 7 },
            Sale { region: "north", amount: 7 },
            Sale { region: "west", amount: 3 },
            Sale { region: "south", amount: -2 },
        ];

        let got = render_region_report(&sales);
        let expected = "east: 7\nnorth: 7\nwest: 7\nsouth: 0\n";
        assert_eq!(got, expected);
    }

    #[test]
    fn ignores_non_positive_sales_but_keeps_seen_regions() {
        let sales = [
            Sale { region: "central", amount: 0 },
            Sale { region: "coast", amount: -3 },
            Sale { region: "central", amount: 5 },
            Sale { region: "coast", amount: 2 },
            Sale { region: "delta", amount: -1 },
        ];

        let got = render_region_report(&sales);
        let expected = "central: 5\ncoast: 2\ndelta: 0\n";
        assert_eq!(got, expected);
    }
}
