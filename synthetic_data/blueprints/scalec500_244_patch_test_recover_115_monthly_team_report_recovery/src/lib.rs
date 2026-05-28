use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub team: &'static str,
    pub month: u8,
    pub opened: u32,
    pub closed: u32,
}

pub fn build_report(events: &[Event]) -> String {
    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for e in events {
        let entry = totals.entry(e.team).or_insert((0, 0));
        entry.0 += e.opened;
        entry.1 += e.closed;
    }

    let mut out = String::new();
    for (team, (opened, closed)) in totals {
        let net = opened.saturating_sub(closed);
        out.push_str(&format!("{team}: opened={opened}, closed={closed}, net={net}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_by_team_sums_only_months_1_to_12_and_sorts_by_net_desc_then_team() {
        let events = [
            Event { team: "Beta", month: 1, opened: 4, closed: 1 },
            Event { team: "Alpha", month: 2, opened: 5, closed: 5 },
            Event { team: "Beta", month: 13, opened: 100, closed: 0 },
            Event { team: "Alpha", month: 7, opened: 3, closed: 1 },
            Event { team: "Gamma", month: 3, opened: 2, closed: 0 },
        ];

        let report = build_report(&events);
        let expected = concat!(
            "Beta: opened=4, closed=1, net=3\n",
            "Alpha: opened=8, closed=6, net=2\n",
            "Gamma: opened=2, closed=0, net=2\n"
        );

        assert_eq!(report, expected);
    }

    #[test]
    fn suppresses_zero_activity_teams_after_filtering_invalid_months() {
        let events = [
            Event { team: "Ops", month: 0, opened: 9, closed: 1 },
            Event { team: "Ops", month: 14, opened: 3, closed: 2 },
            Event { team: "Core", month: 4, opened: 1, closed: 1 },
            Event { team: "UI", month: 6, opened: 2, closed: 1 },
        ];

        let report = build_report(&events);
        let expected = "UI: opened=2, closed=1, net=1\n";

        assert_eq!(report, expected);
    }
}
