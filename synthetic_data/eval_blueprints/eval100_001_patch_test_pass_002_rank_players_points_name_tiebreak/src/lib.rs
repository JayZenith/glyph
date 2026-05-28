#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub penalties: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .then(a.penalties.cmp(&b.penalties))
            .then(a.name.cmp(&b.name))
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
    fn sorts_by_score_descending() {
        let players = vec![p("Mia", 8, 2), p("Ava", 11, 5), p("Noah", 10, 1)];
        assert_eq!(leaderboard(&players), vec!["Ava", "Noah", "Mia"]);
    }

    #[test]
    fn breaks_ties_by_fewer_penalties() {
        let players = vec![p("Kai", 15, 3), p("Lia", 15, 1), p("Omar", 15, 2)];
        assert_eq!(leaderboard(&players), vec!["Lia", "Omar", "Kai"]);
    }

    #[test]
    fn breaks_remaining_ties_by_name() {
        let players = vec![p("Zoe", 20, 0), p("Ana", 20, 0), p("Ben", 20, 0)];
        assert_eq!(leaderboard(&players), vec!["Ana", "Ben", "Zoe"]);
    }

    #[test]
    fn mixed_order_uses_all_tiebreaks() {
        let players = vec![
            p("Nina", 12, 2),
            p("Eli", 14, 4),
            p("Adam", 14, 1),
            p("Zara", 14, 1),
            p("Bea", 12, 0),
        ];
        assert_eq!(leaderboard(&players), vec!["Adam", "Zara", "Eli", "Bea", "Nina"]);
    }
}
