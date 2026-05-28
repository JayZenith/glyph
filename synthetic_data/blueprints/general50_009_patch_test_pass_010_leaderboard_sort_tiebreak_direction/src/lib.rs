use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by_key(|p| (Reverse(p.score), Reverse(p.wins), Reverse(p.name.as_str())));
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
            p("zoe", 15, 1),
            p("amy", 20, 2),
            p("bob", 20, 3),
            p("cara", 20, 3),
            p("dave", 15, 4),
        ];

        assert_eq!(
            leaderboard(&players),
            vec!["bob", "cara", "amy", "dave", "zoe"]
        );
    }

    #[test]
    fn name_tiebreak_is_alphabetical_not_reverse() {
        let players = vec![
            p("mila", 9, 1),
            p("anna", 9, 1),
            p("olga", 9, 1),
        ];

        assert_eq!(leaderboard(&players), vec!["anna", "mila", "olga"]);
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let players: Vec<Player> = vec![];
        assert!(leaderboard(&players).is_empty());
    }
}
