use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Ticket<'a> {
    pub team: &'a str,
    pub status: &'a str,
    pub count: u32,
}

pub fn team_status_report(tickets: &[Ticket<'_>], team: &str) -> Vec<String> {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for ticket in tickets {
        if ticket.team == team {
            *totals.entry(ticket.status).or_insert(0) += 1;
        }
    }

    let order = ["open", "blocked", "closed"];
    let mut lines = Vec::new();

    for status in order {
        let value = totals.get(status).copied().unwrap_or(0);
        lines.push(format!("{}={}", status, value));
    }

    lines.push(format!("total={}", totals.len()));
    lines
}

#[cfg(test)]
mod tests {
    use super::{team_status_report, Ticket};

    #[test]
    fn reports_sum_counts_in_fixed_order_and_total_items() {
        let tickets = [
            Ticket { team: "alpha", status: "open", count: 2 },
            Ticket { team: "alpha", status: "blocked", count: 1 },
            Ticket { team: "alpha", status: "open", count: 3 },
            Ticket { team: "beta", status: "closed", count: 9 },
            Ticket { team: "alpha", status: "closed", count: 4 },
        ];

        let lines = team_status_report(&tickets, "alpha");
        assert_eq!(lines, vec!["open=5", "blocked=1", "closed=4", "total=10"]);
    }

    #[test]
    fn skips_zero_statuses_but_keeps_defined_order() {
        let tickets = [
            Ticket { team: "ops", status: "closed", count: 2 },
            Ticket { team: "ops", status: "closed", count: 1 },
            Ticket { team: "sales", status: "open", count: 7 },
        ];

        let lines = team_status_report(&tickets, "ops");
        assert_eq!(lines, vec!["closed=3", "total=3"]);
    }

    #[test]
    fn unknown_team_yields_only_total_zero() {
        let tickets = [
            Ticket { team: "alpha", status: "open", count: 2 },
            Ticket { team: "alpha", status: "blocked", count: 1 },
        ];

        let lines = team_status_report(&tickets, "gamma");
        assert_eq!(lines, vec!["total=0"]);
    }
}
