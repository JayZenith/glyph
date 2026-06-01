use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub penalties: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by_key(|p| (Reverse(p.score), p.penalties, Reverse(p.name.clone())));
    items.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32, penalties: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            penalties,
        }
    }

    #[test]
    fn sorts_by_score_then_penalties_then_name() {
        let players = vec![
            p("zoe", 15, 1),
            p("amy", 20, 3),
            p("bob", 20, 1),
            p("cara", 20, 1),
            p("dave", 15, 0),
        ];

        assert_eq!(
            leaderboard(&players),
            vec!["bob", "cara", "amy", "dave", "zoe"]
        );
    }

    #[test]
    fn keeps_alphabetical_order_for_exact_metric_ties() {
        let players = vec![
            p("mila", 8, 2),
            p("anna", 8, 2),
            p("olga", 8, 2),
        ];

        assert_eq!(leaderboard(&players), vec!["anna", "mila", "olga"]);
    }
}
