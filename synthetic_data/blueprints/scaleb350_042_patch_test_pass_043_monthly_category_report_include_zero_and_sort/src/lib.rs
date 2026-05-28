#[derive(Clone, Debug)]
pub struct Txn {
    pub month: u8,
    pub category: &'static str,
    pub amount_cents: i64,
    pub settled: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CategoryTotal {
    pub category: &'static str,
    pub total_cents: i64,
}

pub fn monthly_report(txns: &[Txn], month: u8) -> Vec<CategoryTotal> {
    let mut totals: Vec<CategoryTotal> = Vec::new();

    for txn in txns {
        if txn.month != month || !txn.settled || txn.amount_cents <= 0 {
            continue;
        }

        if let Some(existing) = totals.iter_mut().find(|row| row.category == txn.category) {
            existing.total_cents += txn.amount_cents;
        } else {
            totals.push(CategoryTotal {
                category: txn.category,
                total_cents: txn.amount_cents,
            });
        }
    }

    totals.sort_by(|a, b| a.category.cmp(b.category));
    totals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_only_counts_settled_positive_amounts_for_month() {
        let txns = vec![
            Txn { month: 5, category: "food", amount_cents: 2500, settled: true },
            Txn { month: 5, category: "food", amount_cents: -500, settled: true },
            Txn { month: 5, category: "travel", amount_cents: 3000, settled: false },
            Txn { month: 4, category: "food", amount_cents: 1000, settled: true },
            Txn { month: 5, category: "books", amount_cents: 1200, settled: true },
        ];

        assert_eq!(
            monthly_report(&txns, 5),
            vec![
                CategoryTotal { category: "food", total_cents: 2500 },
                CategoryTotal { category: "books", total_cents: 1200 },
            ]
        );
    }

    #[test]
    fn report_keeps_zero_total_categories_seen_in_month_and_sorts_with_tiebreaker() {
        let txns = vec![
            Txn { month: 7, category: "games", amount_cents: -1000, settled: true },
            Txn { month: 7, category: "games", amount_cents: 0, settled: true },
            Txn { month: 7, category: "books", amount_cents: 1500, settled: true },
            Txn { month: 7, category: "garden", amount_cents: 1500, settled: true },
            Txn { month: 7, category: "garden", amount_cents: -300, settled: true },
            Txn { month: 7, category: "books", amount_cents: -200, settled: false },
            Txn { month: 6, category: "music", amount_cents: 999, settled: true },
        ];

        assert_eq!(
            monthly_report(&txns, 7),
            vec![
                CategoryTotal { category: "books", total_cents: 1500 },
                CategoryTotal { category: "garden", total_cents: 1500 },
                CategoryTotal { category: "games", total_cents: 0 },
            ]
        );
    }
}
