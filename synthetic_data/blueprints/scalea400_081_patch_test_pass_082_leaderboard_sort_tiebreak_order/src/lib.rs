use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub penalties: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by_key(|p| (p.score, Reverse(p.penalties), p.name.clone()));
    rows.into_iter().map(|p| p.name).collect()
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
    fn higher_scores_rank_first() {
        let players = vec![p("Ada", 8, 1), p("Bob", 11, 9), p("Cy", 5, 0)];
        assert_eq!(leaderboard(&players), vec!["Bob", "Ada", "Cy"]);
    }

    #[test]
    fn lower_penalties_win_ties_on_score() {
        let players = vec![p("Ada", 10, 3), p("Bob", 10, 1), p("Cy", 9, 0)];
        assert_eq!(leaderboard(&players), vec!["Bob", "Ada", "Cy"]);
    }

    #[test]
    fn names_break_full_ties_alphabetically() {
        let players = vec![p("Zoe", 7, 2), p("Ada", 7, 2), p("Mia", 7, 2)];
        assert_eq!(leaderboard(&players), vec!["Ada", "Mia", "Zoe"]);
    }

    #[test]
    fn mixed_order_respects_all_tiebreaks() {
        let players = vec![
            p("Zed", 12, 4),
            p("Ava", 12, 2),
            p("Ben", 15, 8),
            p("Cal", 12, 2),
            p("Dee", 15, 1),
        ];
        assert_eq!(leaderboard(&players), vec!["Dee", "Ben", "Ava", "Cal", "Zed"]);
    }
}
