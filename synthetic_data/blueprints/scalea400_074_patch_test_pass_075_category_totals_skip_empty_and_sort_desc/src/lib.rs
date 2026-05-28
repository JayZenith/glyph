#[derive(Clone, Copy, Debug)]
pub struct Entry {
    pub category: &'static str,
    pub amount: i32,
    pub active: bool,
}

pub fn summarize(entries: &[Entry]) -> String {
    let mut groups: Vec<(&'static str, i32, usize)> = Vec::new();

    for e in entries {
        if !e.active {
            continue;
        }

        if let Some((_, total, count)) = groups.iter_mut().find(|(name, _, _)| *name == e.category) {
            *total += e.amount;
            *count += 1;
        } else {
            groups.push((e.category, e.amount, 1));
        }
    }

    groups.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::new();
    for (idx, (name, total, count)) in groups.iter().enumerate() {
        if idx > 0 {
            out.push('\n');
        }
        out.push_str(&format!("{}: total={}, count={}", name, total, count));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_active_entries_and_orders_by_total_desc() {
        let entries = [
            Entry { category: "tools", amount: 10, active: true },
            Entry { category: "food", amount: 5, active: true },
            Entry { category: "tools", amount: 7, active: true },
            Entry { category: "food", amount: 4, active: false },
            Entry { category: "travel", amount: 12, active: true },
        ];

        assert_eq!(
            summarize(&entries),
            "tools: total=17, count=2\ntravel: total=12, count=1\nfood: total=5, count=1"
        );
    }

    #[test]
    fn skips_zero_amount_entries_and_uses_name_tiebreaker() {
        let entries = [
            Entry { category: "beta", amount: 3, active: true },
            Entry { category: "alpha", amount: 3, active: true },
            Entry { category: "alpha", amount: 0, active: true },
            Entry { category: "gamma", amount: 0, active: true },
            Entry { category: "beta", amount: 0, active: false },
        ];

        assert_eq!(
            summarize(&entries),
            "alpha: total=3, count=1\nbeta: total=3, count=1"
        );
    }
}
