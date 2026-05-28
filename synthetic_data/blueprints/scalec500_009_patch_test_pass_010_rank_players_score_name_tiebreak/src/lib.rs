use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by_key(|p| (Reverse(p.score), p.name.clone(), Reverse(p.wins)));
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
            p("zoe", 15, 2),
            p("amy", 20, 1),
            p("bob", 20, 3),
            p("cara", 20, 3),
            p("dan", 15, 5),
        ];

        let names = leaderboard(&players);
        assert_eq!(names, vec!["bob", "cara", "amy", "dan", "zoe"]);
    }

    #[test]
    fn uses_name_only_as_final_tiebreak() {
        let players = vec![
            p("mila", 7, 4),
            p("anna", 7, 4),
            p("nora", 7, 5),
        ];

        let names = leaderboard(&players);
        assert_eq!(names, vec!["nora", "anna", "mila"]);
    }
}
