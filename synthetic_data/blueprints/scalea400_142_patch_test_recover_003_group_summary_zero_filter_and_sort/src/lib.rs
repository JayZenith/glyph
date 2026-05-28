use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub team: &'static str,
    pub delta: i32,
}

pub fn summarize(entries: &[Entry]) -> Vec<String> {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();
    for entry in entries {
        *totals.entry(entry.team).or_insert(0) += entry.delta;
    }

    let mut rows: Vec<(&str, i32)> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(team, total)| format!("{}:{}", team, total))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_total_desc_then_team_and_skips_zero_totals() {
        let entries = [
            Entry { team: "blue", delta: 2 },
            Entry { team: "red", delta: 3 },
            Entry { team: "blue", delta: 3 },
            Entry { team: "green", delta: 1 },
            Entry { team: "red", delta: -3 },
            Entry { team: "amber", delta: 5 },
        ];

        assert_eq!(
            summarize(&entries),
            vec!["amber:5", "blue:5", "green:1"]
        );
    }

    #[test]
    fn tie_breaks_equal_totals_by_team_name() {
        let entries = [
            Entry { team: "zeta", delta: 4 },
            Entry { team: "alpha", delta: 4 },
            Entry { team: "beta", delta: 2 },
            Entry { team: "beta", delta: 2 },
        ];

        assert_eq!(
            summarize(&entries),
            vec!["alpha:4", "beta:4", "zeta:4"]
        );
    }
}
