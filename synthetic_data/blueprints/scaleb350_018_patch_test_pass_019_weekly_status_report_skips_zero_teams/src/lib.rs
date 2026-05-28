use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub team: &'static str,
    pub status: &'static str,
    pub count: u32,
}

pub fn render_status_report(records: &[Record]) -> String {
    let mut grouped: BTreeMap<&str, (u32, u32, u32)> = BTreeMap::new();

    for record in records {
        let entry = grouped.entry(record.team).or_insert((0, 0, 0));
        match record.status {
            "open" => entry.0 += record.count,
            "closed" => entry.1 += record.count,
            "blocked" => entry.2 += record.count,
            _ => {}
        }
    }

    let mut lines = Vec::new();
    let mut grand_total = 0;

    for (team, (open, closed, blocked)) in grouped {
        let total = open + closed + blocked;
        grand_total += total;
        lines.push(format!(
            "{team}: total={total} open={open} closed={closed} blocked={blocked}"
        ));
    }

    lines.push(format!("GRAND TOTAL: {grand_total}"));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_by_team_and_sorts_alphabetically() {
        let records = vec![
            Record { team: "beta", status: "closed", count: 2 },
            Record { team: "alpha", status: "open", count: 3 },
            Record { team: "beta", status: "blocked", count: 1 },
            Record { team: "alpha", status: "closed", count: 4 },
        ];

        let report = render_status_report(&records);
        assert_eq!(
            report,
            "alpha: total=7 open=3 closed=4 blocked=0\nbeta: total=3 open=0 closed=2 blocked=1\nGRAND TOTAL: 10"
        );
    }

    #[test]
    fn skips_teams_with_zero_total_even_if_only_unknown_statuses_exist() {
        let records = vec![
            Record { team: "alpha", status: "open", count: 2 },
            Record { team: "ghost", status: "ignored", count: 9 },
            Record { team: "beta", status: "blocked", count: 1 },
            Record { team: "beta", status: "mystery", count: 5 },
        ];

        let report = render_status_report(&records);
        assert_eq!(
            report,
            "alpha: total=2 open=2 closed=0 blocked=0\nbeta: total=1 open=0 closed=0 blocked=1\nGRAND TOTAL: 3"
        );
    }

    #[test]
    fn returns_only_grand_total_when_nothing_countable_exists() {
        let records = vec![
            Record { team: "ghost", status: "unknown", count: 4 },
            Record { team: "phantom", status: "other", count: 7 },
        ];

        let report = render_status_report(&records);
        assert_eq!(report, "GRAND TOTAL: 0");
    }
}
