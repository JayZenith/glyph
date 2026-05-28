#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: &'static str,
    pub score: u32,
    pub penalties: u32,
}

pub fn rank_entries(entries: &[Entry]) -> Vec<Entry> {
    let mut out = entries.to_vec();
    out.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(a.penalties.cmp(&b.penalties))
            .then(a.name.cmp(b.name))
    });
    out
}

#[cfg(test)]
mod tests {
    use super::{rank_entries, Entry};

    fn names(entries: &[Entry]) -> Vec<&'static str> {
        entries.iter().map(|e| e.name).collect()
    }

    #[test]
    fn sorts_by_score_descending() {
        let items = vec![
            Entry { name: "gamma", score: 7, penalties: 2 },
            Entry { name: "alpha", score: 10, penalties: 9 },
            Entry { name: "beta", score: 8, penalties: 1 },
        ];

        let ranked = rank_entries(&items);
        assert_eq!(names(&ranked), vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn breaks_score_ties_by_lower_penalties() {
        let items = vec![
            Entry { name: "oak", score: 12, penalties: 4 },
            Entry { name: "elm", score: 12, penalties: 1 },
            Entry { name: "ash", score: 12, penalties: 3 },
        ];

        let ranked = rank_entries(&items);
        assert_eq!(names(&ranked), vec!["elm", "ash", "oak"]);
    }

    #[test]
    fn breaks_full_ties_alphabetically() {
        let items = vec![
            Entry { name: "zoe", score: 9, penalties: 2 },
            Entry { name: "amy", score: 9, penalties: 2 },
            Entry { name: "max", score: 9, penalties: 2 },
        ];

        let ranked = rank_entries(&items);
        assert_eq!(names(&ranked), vec!["amy", "max", "zoe"]);
    }

    #[test]
    fn handles_mixed_ranking_rules_together() {
        let items = vec![
            Entry { name: "kai", score: 15, penalties: 3 },
            Entry { name: "ben", score: 15, penalties: 1 },
            Entry { name: "ava", score: 17, penalties: 5 },
            Entry { name: "dan", score: 15, penalties: 1 },
            Entry { name: "eve", score: 11, penalties: 0 },
        ];

        let ranked = rank_entries(&items);
        assert_eq!(names(&ranked), vec!["ava", "ben", "dan", "kai", "eve"]);
    }
}
