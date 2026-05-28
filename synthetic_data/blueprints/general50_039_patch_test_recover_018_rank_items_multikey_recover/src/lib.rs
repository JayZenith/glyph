#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: u32,
    pub score: i32,
    pub wins: u32,
    pub penalty: u32,
}

pub fn leaderboard(items: &[Item]) -> Vec<u32> {
    let mut v = items.to_vec();
    v.sort_by(|a, b| {
        b.score.cmp(&a.score)
            .then_with(|| a.penalty.cmp(&b.penalty))
            .then_with(|| a.id.cmp(&b.id))
    });
    v.into_iter().map(|x| x.id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_score_then_wins_then_penalty_then_id() {
        let items = vec![
            Item { id: 30, score: 50, wins: 5, penalty: 7 },
            Item { id: 10, score: 50, wins: 7, penalty: 20 },
            Item { id: 20, score: 50, wins: 7, penalty: 10 },
            Item { id: 40, score: 45, wins: 99, penalty: 0 },
        ];
        assert_eq!(leaderboard(&items), vec![20, 10, 30, 40]);
    }

    #[test]
    fn prefers_lower_id_only_after_all_other_fields_tie() {
        let items = vec![
            Item { id: 9, score: 12, wins: 3, penalty: 4 },
            Item { id: 1, score: 12, wins: 3, penalty: 4 },
            Item { id: 5, score: 12, wins: 3, penalty: 5 },
        ];
        assert_eq!(leaderboard(&items), vec![1, 9, 5]);
    }

    #[test]
    fn higher_wins_beat_lower_penalty_when_scores_match() {
        let items = vec![
            Item { id: 1, score: 100, wins: 2, penalty: 0 },
            Item { id: 2, score: 100, wins: 3, penalty: 99 },
            Item { id: 3, score: 100, wins: 3, penalty: 100 },
        ];
        assert_eq!(leaderboard(&items), vec![2, 3, 1]);
    }
}
