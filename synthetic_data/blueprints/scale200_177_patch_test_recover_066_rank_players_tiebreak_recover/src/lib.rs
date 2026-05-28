#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub wins: u32,
    pub score_diff: i32,
}

pub fn leaderboard(players: &[Player]) -> Vec<&'static str> {
    let mut items: Vec<&Player> = players.iter().collect();
    items.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then_with(|| a.name.cmp(b.name))
    });
    items.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_wins_then_score_diff_then_name() {
        let players = vec![
            Player { name: "Lia", wins: 4, score_diff: 7 },
            Player { name: "Ava", wins: 5, score_diff: 1 },
            Player { name: "Mia", wins: 5, score_diff: 9 },
            Player { name: "Zoe", wins: 4, score_diff: 10 },
        ];

        assert_eq!(leaderboard(&players), vec!["Mia", "Ava", "Zoe", "Lia"]);
    }

    #[test]
    fn uses_name_as_final_tiebreaker() {
        let players = vec![
            Player { name: "Cara", wins: 3, score_diff: 2 },
            Player { name: "Bella", wins: 3, score_diff: 2 },
            Player { name: "Anna", wins: 3, score_diff: 2 },
        ];

        assert_eq!(leaderboard(&players), vec!["Anna", "Bella", "Cara"]);
    }
}
