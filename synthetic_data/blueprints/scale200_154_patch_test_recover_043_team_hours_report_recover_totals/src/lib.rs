#[derive(Clone, Copy)]
pub struct Entry {
    pub team: &'static str,
    pub hours: u32,
    pub active: bool,
}

pub fn team_report(entries: &[Entry]) -> String {
    let mut rows: Vec<(&str, u32, u32)> = Vec::new();

    for e in entries {
        let mut found = false;
        for row in rows.iter_mut() {
            if row.0 == e.team {
                row.1 += e.hours;
                row.2 += 1;
                found = true;
                break;
            }
        }
        if !found {
            rows.push((e.team, e.hours, 1));
        }
    }

    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::new();
    for (team, hours, active_count) in rows {
        out.push_str(&format!("{}: {}h ({} active)\n", team, hours, active_count));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{team_report, Entry};

    #[test]
    fn aggregates_only_active_members_and_sorts_by_total_desc_then_name() {
        let entries = [
            Entry { team: "Blue", hours: 5, active: true },
            Entry { team: "Red", hours: 8, active: true },
            Entry { team: "Blue", hours: 7, active: false },
            Entry { team: "Red", hours: 4, active: true },
            Entry { team: "Amber", hours: 9, active: true },
            Entry { team: "Amber", hours: 3, active: false },
        ];

        let report = team_report(&entries);
        let expected = "Red: 12h (2 active)\nAmber: 9h (1 active)\nBlue: 5h (1 active)\n";
        assert_eq!(report, expected);
    }

    #[test]
    fn ties_on_hours_break_by_team_name() {
        let entries = [
            Entry { team: "Ops", hours: 4, active: true },
            Entry { team: "QA", hours: 4, active: true },
            Entry { team: "Ops", hours: 2, active: false },
            Entry { team: "QA", hours: 1, active: false },
        ];

        let report = team_report(&entries);
        let expected = "Ops: 4h (1 active)\nQA: 4h (1 active)\n";
        assert_eq!(report, expected);
    }
}
