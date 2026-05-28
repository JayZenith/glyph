#[derive(Clone, Debug)]
pub struct Entry {
    pub dept: &'static str,
    pub employee: &'static str,
    pub hours: i32,
    pub active: bool,
}

pub fn build_report(entries: &[Entry]) -> String {
    let mut rows: Vec<(&str, usize, i32)> = Vec::new();

    for entry in entries {
        match rows.iter_mut().find(|row| row.0 == entry.dept) {
            Some(row) => {
                row.1 += 1;
                row.2 += entry.hours;
            }
            None => rows.push((entry.dept, 1, entry.hours)),
        }
    }

    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(dept, people, hours)| format!("{}: {} active, {} hours", dept, people, hours))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_only_active_departments_and_sums_positive_hours() {
        let entries = [
            Entry { dept: "Ops", employee: "Ava", hours: 5, active: true },
            Entry { dept: "Sales", employee: "Ben", hours: 3, active: true },
            Entry { dept: "Ops", employee: "Cara", hours: -2, active: true },
            Entry { dept: "Sales", employee: "Drew", hours: 0, active: false },
            Entry { dept: "Legal", employee: "Eli", hours: 4, active: false },
            Entry { dept: "Ops", employee: "Finn", hours: 0, active: true },
        ];

        let got = build_report(&entries);
        let expected = "Ops: 3 active, 5 hours\nSales: 1 active, 3 hours";
        assert_eq!(got, expected);
    }

    #[test]
    fn sorts_by_total_hours_desc_then_department_name() {
        let entries = [
            Entry { dept: "Beta", employee: "A", hours: 4, active: true },
            Entry { dept: "Alpha", employee: "B", hours: 4, active: true },
            Entry { dept: "Gamma", employee: "C", hours: 2, active: true },
        ];

        let got = build_report(&entries);
        let expected = "Alpha: 1 active, 4 hours\nBeta: 1 active, 4 hours\nGamma: 1 active, 2 hours";
        assert_eq!(got, expected);
    }
}
