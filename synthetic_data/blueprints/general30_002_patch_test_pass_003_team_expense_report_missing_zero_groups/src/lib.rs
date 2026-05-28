use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expense {
    pub team: &'static str,
    pub amount: u32,
    pub paid: bool,
}

pub fn expense_report(expenses: &[Expense], teams: &[&str]) -> String {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for expense in expenses {
        if expense.paid && teams.contains(&expense.team) {
            *totals.entry(expense.team).or_insert(0) += expense.amount;
        }
    }

    teams
        .iter()
        .filter_map(|team| totals.get(team).map(|total| format!("{}:{}", team, total)))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_zero_total_for_requested_team_without_paid_items() {
        let expenses = [
            Expense { team: "ops", amount: 30, paid: false },
            Expense { team: "eng", amount: 70, paid: true },
            Expense { team: "sales", amount: 25, paid: true },
        ];

        let report = expense_report(&expenses, &["ops", "eng", "hr"]);
        assert_eq!(report, "ops:0\neng:70\nhr:0");
    }

    #[test]
    fn preserves_requested_order_and_ignores_unrequested_teams() {
        let expenses = [
            Expense { team: "sales", amount: 10, paid: true },
            Expense { team: "eng", amount: 20, paid: true },
            Expense { team: "sales", amount: 5, paid: true },
            Expense { team: "ops", amount: 99, paid: true },
        ];

        let report = expense_report(&expenses, &["sales", "eng"]);
        assert_eq!(report, "sales:15\neng:20");
    }
}
