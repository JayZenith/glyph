use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by_key(|p| (Reverse(p.score), Reverse(p.wins), Reverse(p.name.clone())));
    items.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            wins,
        }
    }

    #[test]
    fn sorts_by_score_then_wins_then_name() {
        let players = vec![
            p("zoe", 12, 3),
            p("amy", 15, 1),
            p("bob", 15, 4),
            p("cara", 15, 4),
            p("dan", 12, 8),
        ];

        assert_eq!(
            leaderboard(&players),
            vec!["bob", "cara", "amy", "dan", "zoe"]
        );
    }

    #[test]
    fn alphabetical_name_tiebreak_is_ascending() {
        let players = vec![
            p("mila", 20, 5),
            p("ava", 20, 5),
            p("liam", 20, 5),
        ];

        assert_eq!(leaderboard(&players), vec!["ava", "liam", "mila"]);
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let players = vec![];
        let ranked = leaderboard(&players);
        assert!(ranked.is_empty());
    }
}
