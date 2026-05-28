#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub solved: u32,
    pub penalty: u32,
    pub last_submit_minute: u32,
}

pub fn leaderboard(entries: &[Entry]) -> Vec<String> {
    let mut items = entries.to_vec();
    items.sort_by(|a, b| {
        b.solved
            .cmp(&a.solved)
            .then_with(|| a.penalty.cmp(&b.penalty))
    });
    items.into_iter().map(|e| e.name).collect()
}

#[cfg(test)]
mod tests {
    use super::{leaderboard, Entry};

    fn e(name: &str, solved: u32, penalty: u32, last_submit_minute: u32) -> Entry {
        Entry {
            name: name.to_string(),
            solved,
            penalty,
            last_submit_minute,
        }
    }

    #[test]
    fn sorts_by_solved_then_penalty_then_last_submit_then_name() {
        let rows = vec![
            e("zoe", 4, 300, 90),
            e("amy", 5, 500, 200),
            e("bob", 5, 500, 120),
            e("cyd", 5, 400, 300),
            e("dan", 5, 500, 120),
        ];

        assert_eq!(
            leaderboard(&rows),
            vec!["cyd", "bob", "dan", "amy", "zoe"]
        );
    }

    #[test]
    fn keeps_duplicate_names_as_separate_entries_and_uses_all_tiebreaks() {
        let rows = vec![
            e("sam", 3, 200, 80),
            e("sam", 3, 200, 70),
            e("alex", 3, 200, 70),
            e("max", 2, 10, 1),
        ];

        assert_eq!(
            leaderboard(&rows),
            vec!["alex", "sam", "sam", "max"]
        );
    }
}
