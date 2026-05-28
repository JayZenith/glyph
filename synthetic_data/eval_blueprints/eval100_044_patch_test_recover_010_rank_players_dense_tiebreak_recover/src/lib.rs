#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub wins: u32,
    pub score_diff: i32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| a.name.cmp(&b.name));
    items.reverse();

    items
        .into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} [{}|{}]", i + 1, p.name, p.wins, p.score_diff))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, wins: u32, score_diff: i32) -> Player {
        Player {
            name: name.to_string(),
            wins,
            score_diff,
        }
    }

    #[test]
    fn sorts_by_wins_then_diff_then_name() {
        let players = vec![
            p("Zoe", 3, 5),
            p("Ava", 5, 1),
            p("Liam", 5, 10),
            p("Mia", 5, 10),
            p("Noah", 3, 20),
        ];

        assert_eq!(
            leaderboard(&players),
            vec![
                "1. Liam [5|10]",
                "1. Mia [5|10]",
                "2. Ava [5|1]",
                "3. Noah [3|20]",
                "4. Zoe [3|5]",
            ]
        );
    }

    #[test]
    fn uses_dense_ranks_for_ties() {
        let players = vec![
            p("Bea", 4, 0),
            p("Ana", 4, 0),
            p("Cid", 2, 7),
            p("Dan", 2, 7),
            p("Eli", 1, 9),
        ];

        assert_eq!(
            leaderboard(&players),
            vec![
                "1. Ana [4|0]",
                "1. Bea [4|0]",
                "2. Cid [2|7]",
                "2. Dan [2|7]",
                "3. Eli [1|9]",
            ]
        );
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let players: Vec<Player> = vec![];
        assert!(leaderboard(&players).is_empty());
    }
}
