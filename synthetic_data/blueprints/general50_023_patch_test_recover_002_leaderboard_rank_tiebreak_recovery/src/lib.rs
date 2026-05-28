#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub score: u32,
    pub penalty: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedEntry {
    pub rank: usize,
    pub name: String,
    pub score: u32,
    pub penalty: u32,
}

pub fn leaderboard(entries: &[Entry]) -> Vec<RankedEntry> {
    let mut items = entries.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| a.penalty.cmp(&b.penalty))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(i, e)| RankedEntry {
            rank: i + 1,
            name: e.name,
            score: e.score,
            penalty: e.penalty,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(name: &str, score: u32, penalty: u32) -> Entry {
        Entry {
            name: name.to_string(),
            score,
            penalty,
        }
    }

    #[test]
    fn sorts_by_score_then_penalty_then_name() {
        let out = leaderboard(&[
            e("zoe", 10, 40),
            e("amy", 10, 20),
            e("bob", 10, 20),
            e("dan", 12, 80),
            e("ivy", 9, 1),
        ]);

        let names: Vec<_> = out.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["dan", "amy", "bob", "zoe", "ivy"]);
    }

    #[test]
    fn equal_score_and_penalty_share_rank_with_gaps() {
        let out = leaderboard(&[
            e("bob", 10, 20),
            e("amy", 10, 20),
            e("zoe", 10, 40),
            e("dan", 8, 10),
            e("eve", 8, 10),
        ]);

        let pairs: Vec<_> = out.into_iter().map(|r| (r.name, r.rank)).collect();
        assert_eq!(
            pairs,
            vec![
                ("amy".to_string(), 1),
                ("bob".to_string(), 1),
                ("zoe".to_string(), 3),
                ("dan".to_string(), 4),
                ("eve".to_string(), 4),
            ]
        );
    }

    #[test]
    fn empty_input_returns_empty() {
        assert!(leaderboard(&[]).is_empty());
    }
}
