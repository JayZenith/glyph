use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account<'a> {
    pub team: &'a str,
    pub active: bool,
    pub seats: u32,
}

pub fn seats_by_team(accounts: &[Account<'_>]) -> Vec<(String, u32)> {
    let mut totals: BTreeMap<String, u32> = BTreeMap::new();

    for account in accounts {
        *totals.entry(account.team.to_string()).or_insert(0) += account.seats;
    }

    totals.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_and_sums_active_accounts() {
        let accounts = vec![
            Account { team: "alpha", active: true, seats: 3 },
            Account { team: "beta", active: true, seats: 2 },
            Account { team: "alpha", active: true, seats: 4 },
        ];

        assert_eq!(
            seats_by_team(&accounts),
            vec![
                ("alpha".to_string(), 7),
                ("beta".to_string(), 2),
            ]
        );
    }

    #[test]
    fn skips_inactive_accounts_from_totals() {
        let accounts = vec![
            Account { team: "alpha", active: true, seats: 3 },
            Account { team: "alpha", active: false, seats: 10 },
            Account { team: "beta", active: false, seats: 7 },
            Account { team: "beta", active: true, seats: 2 },
        ];

        assert_eq!(
            seats_by_team(&accounts),
            vec![
                ("alpha".to_string(), 3),
                ("beta".to_string(), 2),
            ]
        );
    }

    #[test]
    fn omits_teams_with_only_inactive_accounts() {
        let accounts = vec![
            Account { team: "alpha", active: false, seats: 5 },
            Account { team: "beta", active: true, seats: 1 },
        ];

        assert_eq!(
            seats_by_team(&accounts),
            vec![
                ("beta".to_string(), 1),
            ]
        );
    }
}
