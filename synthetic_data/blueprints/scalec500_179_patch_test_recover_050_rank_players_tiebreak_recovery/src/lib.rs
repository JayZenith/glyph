use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub score: u32,
    pub wins: u32,
    pub penalties: u32,
}

pub fn leaderboard(entries: &[Entry]) -> Vec<String> {
    let mut best: HashMap<String, Entry> = HashMap::new();

    for e in entries {
        match best.get(&e.name) {
            Some(existing) if existing.score >= e.score => {}
            _ => {
                best.insert(e.name.clone(), e.clone());
            }
        }
    }

    let mut rows: Vec<Entry> = best.into_values().collect();
    rows.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.penalties.cmp(&b.penalties))
            .then_with(|| a.name.cmp(&b.name))
    });

    rows.into_iter().map(|e| e.name).collect()
}

#[cfg(test)]
mod tests {
    use super::{leaderboard, Entry};

    fn e(name: &str, score: u32, wins: u32, penalties: u32) -> Entry {
        Entry {
            name: name.to_string(),
            score,
            wins,
            penalties,
        }
    }

    #[test]
    fn sorts_by_score_then_wins_then_penalties_then_name() {
        let entries = vec![
            e("zoe", 12, 4, 1),
            e("amy", 15, 2, 3),
            e("bob", 15, 3, 5),
            e("cara", 15, 3, 2),
            e("dina", 15, 3, 2),
        ];

        assert_eq!(
            leaderboard(&entries),
            vec!["cara", "dina", "bob", "amy", "zoe"]
        );
    }

    #[test]
    fn keeps_best_record_per_name_using_full_tiebreaks() {
        let entries = vec![
            e("ivy", 20, 4, 6),
            e("ivy", 20, 5, 7),
            e("ivy", 20, 5, 3),
            e("max", 20, 5, 4),
            e("nia", 19, 9, 0),
        ];

        assert_eq!(leaderboard(&entries), vec!["ivy", "max", "nia"]);
    }

    #[test]
    fn duplicate_name_does_not_replace_better_record_with_worse_same_score() {
        let entries = vec![
            e("leo", 18, 6, 1),
            e("leo", 18, 5, 0),
            e("mia", 18, 6, 2),
        ];

        assert_eq!(leaderboard(&entries), vec!["leo", "mia"]);
    }
}
