#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub wins: u32,
    pub goal_diff: i32,
    pub scored: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        a.wins
            .cmp(&b.wins)
            .then(a.goal_diff.cmp(&b.goal_diff))
            .then(a.scored.cmp(&b.scored))
            .then(a.name.cmp(&b.name))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(idx, p)| format!("{}. {}", idx + 1, p.name))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_primary_then_tiebreakers() {
        let players = vec![
            Player { name: "Lions", wins: 3, goal_diff: 5, scored: 7 },
            Player { name: "Bears", wins: 5, goal_diff: 1, scored: 4 },
            Player { name: "Tigers", wins: 5, goal_diff: 3, scored: 2 },
            Player { name: "Wolves", wins: 5, goal_diff: 3, scored: 8 },
        ];

        assert_eq!(
            leaderboard(&players),
            vec![
                "1. Wolves",
                "2. Tigers",
                "3. Bears",
                "4. Lions",
            ]
        );
    }

    #[test]
    fn falls_back_to_name_ascending_when_all_scores_tie() {
        let players = vec![
            Player { name: "Zephyrs", wins: 2, goal_diff: 0, scored: 4 },
            Player { name: "Astros", wins: 2, goal_diff: 0, scored: 4 },
            Player { name: "Comets", wins: 2, goal_diff: 0, scored: 4 },
        ];

        assert_eq!(
            leaderboard(&players),
            vec![
                "1. Astros",
                "2. Comets",
                "3. Zephyrs",
            ]
        );
    }
}
