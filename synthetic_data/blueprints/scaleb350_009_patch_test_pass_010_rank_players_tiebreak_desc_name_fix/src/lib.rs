use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by_key(|p| (Reverse(p.score), p.wins, p.name.clone()));
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
    fn ranks_by_score_then_wins_then_name() {
        let players = vec![
            p("zoe", 12, 3),
            p("amy", 15, 1),
            p("bob", 15, 4),
            p("cara", 15, 4),
            p("dina", 12, 9),
        ];

        assert_eq!(leaderboard(&players), vec!["bob", "cara", "amy", "dina", "zoe"]);
    }

    #[test]
    fn alphabetical_name_breaks_full_ties() {
        let players = vec![p("mila", 8, 2), p("anna", 8, 2), p("olga", 8, 2)];
        assert_eq!(leaderboard(&players), vec!["anna", "mila", "olga"]);
    }
}
