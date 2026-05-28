use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreEntry {
    pub player: &'static str,
    pub points: u32,
    pub solved: u32,
    pub penalty: u32,
}

pub fn leaderboard(entries: &[ScoreEntry]) -> Vec<&'static str> {
    let mut best: HashMap<&'static str, ScoreEntry> = HashMap::new();

    for entry in entries {
        best.entry(entry.player)
            .and_modify(|cur| {
                if entry.points > cur.points {
                    *cur = entry.clone();
                }
            })
            .or_insert_with(|| entry.clone());
    }

    let mut rows: Vec<ScoreEntry> = best.into_values().collect();
    rows.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| a.player.cmp(&b.player))
    });

    rows.into_iter().map(|e| e.player).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_by_points_then_solved_then_penalty_then_name() {
        let entries = vec![
            ScoreEntry { player: "zoe", points: 200, solved: 4, penalty: 80 },
            ScoreEntry { player: "amy", points: 200, solved: 5, penalty: 70 },
            ScoreEntry { player: "bob", points: 200, solved: 5, penalty: 50 },
            ScoreEntry { player: "dan", points: 200, solved: 5, penalty: 50 },
            ScoreEntry { player: "eve", points: 180, solved: 6, penalty: 20 },
        ];

        assert_eq!(leaderboard(&entries), vec!["bob", "dan", "amy", "zoe", "eve"]);
    }

    #[test]
    fn duplicate_players_keep_best_entry_using_all_tiebreakers() {
        let entries = vec![
            ScoreEntry { player: "ivy", points: 300, solved: 5, penalty: 90 },
            ScoreEntry { player: "ivy", points: 300, solved: 6, penalty: 120 },
            ScoreEntry { player: "max", points: 300, solved: 6, penalty: 130 },
            ScoreEntry { player: "neo", points: 290, solved: 10, penalty: 10 },
            ScoreEntry { player: "ivy", points: 300, solved: 6, penalty: 80 },
        ];

        assert_eq!(leaderboard(&entries), vec!["ivy", "max", "neo"]);
    }

    #[test]
    fn duplicate_players_do_not_replace_with_worse_same_point_entry() {
        let entries = vec![
            ScoreEntry { player: "kai", points: 250, solved: 7, penalty: 40 },
            ScoreEntry { player: "lia", points: 250, solved: 7, penalty: 50 },
            ScoreEntry { player: "kai", points: 250, solved: 6, penalty: 10 },
        ];

        assert_eq!(leaderboard(&entries), vec!["kai", "lia"]);
    }
}
