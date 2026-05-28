#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Txn {
    pub category: &'static str,
    pub cents: i64,
    pub finalized: bool,
    pub refunded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CategoryReport {
    pub category: &'static str,
    pub total_cents: i64,
    pub settled_count: usize,
}

pub fn weekly_report(txns: &[Txn]) -> Vec<CategoryReport> {
    let mut rows: Vec<CategoryReport> = Vec::new();

    for txn in txns {
        if let Some(row) = rows.iter_mut().find(|r| r.category == txn.category) {
            row.total_cents += txn.cents;
            if !txn.refunded {
                row.settled_count += 1;
            }
        } else {
            rows.push(CategoryReport {
                category: txn.category,
                total_cents: txn.cents,
                settled_count: if txn.refunded { 0 } else { 1 },
            });
        }
    }

    rows.sort_by(|a, b| b.total_cents.cmp(&a.total_cents).then_with(|| a.category.cmp(b.category)));
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finalized_only_and_zero_net_categories_omitted() {
        let txns = vec![
            Txn { category: "books", cents: 1200, finalized: true, refunded: false },
            Txn { category: "books", cents: -1200, finalized: true, refunded: true },
            Txn { category: "games", cents: 3000, finalized: false, refunded: false },
            Txn { category: "games", cents: 2500, finalized: true, refunded: false },
            Txn { category: "music", cents: 700, finalized: true, refunded: false },
        ];

        let report = weekly_report(&txns);

        assert_eq!(
            report,
            vec![
                CategoryReport { category: "games", total_cents: 2500, settled_count: 1 },
                CategoryReport { category: "music", total_cents: 700, settled_count: 1 },
            ]
        );
    }

    #[test]
    fn refunded_finalized_transactions_reduce_total_but_not_count() {
        let txns = vec![
            Txn { category: "food", cents: 1500, finalized: true, refunded: false },
            Txn { category: "food", cents: 400, finalized: false, refunded: false },
            Txn { category: "food", cents: -200, finalized: true, refunded: true },
            Txn { category: "travel", cents: 900, finalized: true, refunded: false },
        ];

        let report = weekly_report(&txns);

        assert_eq!(
            report,
            vec![
                CategoryReport { category: "food", total_cents: 1300, settled_count: 1 },
                CategoryReport { category: "travel", total_cents: 900, settled_count: 1 },
            ]
        );
    }
}
