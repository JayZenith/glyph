use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<&'static str> {
    let mut rows: Vec<&Player> = players.iter().collect();
    rows.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .reverse()
            .then(a.wins.cmp(&b.wins).reverse())
            .then_with(|| a.name.cmp(b.name))
    });
    rows.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_by_score_then_wins_then_name_desc() {
        let players = vec![
            Player { name: "Ivy", score: 18, wins: 4 },
            Player { name: "Zoe", score: 20, wins: 3 },
            Player { name: "Ava", score: 20, wins: 3 },
            Player { name: "Mia", score: 20, wins: 5 },
        ];

        assert_eq!(leaderboard(&players), vec!["Mia", "Zoe", "Ava", "Ivy"]);
    }

    #[test]
    fn handles_full_tie_group_with_reverse_alphabetical_tiebreak() {
        let players = vec![
            Player { name: "Ben", score: 7, wins: 1 },
            Player { name: "Ada", score: 7, wins: 1 },
            Player { name: "Kai", score: 7, wins: 1 },
        ];

        assert_eq!(leaderboard(&players), vec!["Kai", "Ben", "Ada"]);
    }

    #[test]
    fn lower_score_never_beats_higher_score_even_with_more_wins() {
        let players = vec![
            Player { name: "Noah", score: 12, wins: 9 },
            Player { name: "Liam", score: 13, wins: 0 },
            Player { name: "Emma", score: 12, wins: 10 },
        ];

        assert_eq!(leaderboard(&players), vec!["Liam", "Emma", "Noah"]);
    }
}
