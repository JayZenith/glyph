#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub penalties: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| b.penalties.cmp(&a.penalties))
            .then_with(|| b.name.cmp(&a.name))
    });

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
    fn orders_by_score_descending() {
        let players = vec![p("Mia", 8, 3), p("Noah", 12, 9), p("Ava", 10, 1)];
        assert_eq!(leaderboard(&players), vec!["Noah", "Ava", "Mia"]);
    }

    #[test]
    fn uses_lower_penalties_as_tiebreak() {
        let players = vec![p("Kai", 10, 4), p("Lia", 10, 1), p("Zoe", 9, 0)];
        assert_eq!(leaderboard(&players), vec!["Lia", "Kai", "Zoe"]);
    }

    #[test]
    fn uses_name_ascending_when_score_and_penalties_match() {
        let players = vec![p("Zed", 7, 2), p("Ana", 7, 2), p("Bob", 7, 2)];
        assert_eq!(leaderboard(&players), vec!["Ana", "Bob", "Zed"]);
    }

    #[test]
    fn combines_all_tiebreak_rules() {
        let players = vec![
            p("Eli", 15, 3),
            p("Ari", 15, 1),
            p("Zia", 15, 1),
            p("Moe", 12, 0),
            p("Bea", 12, 2),
        ];
        assert_eq!(leaderboard(&players), vec!["Ari", "Zia", "Eli", "Moe", "Bea"]);
    }
}
