#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<&'static str> {
    let mut items: Vec<&Player> = players.iter().collect();
    items.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .then(a.wins.cmp(&b.wins))
            .then(a.name.cmp(&b.name))
    });
    items.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_score_then_wins_then_name() {
        let players = vec![
            Player { name: "zoe", score: 12, wins: 4 },
            Player { name: "amy", score: 15, wins: 2 },
            Player { name: "bob", score: 15, wins: 3 },
            Player { name: "eve", score: 15, wins: 3 },
            Player { name: "max", score: 12, wins: 6 },
        ];

        assert_eq!(leaderboard(&players), vec!["bob", "eve", "amy", "max", "zoe"]);
    }

    #[test]
    fn alphabetical_name_breaks_full_ties() {
        let players = vec![
            Player { name: "liam", score: 9, wins: 1 },
            Player { name: "ava", score: 9, wins: 1 },
            Player { name: "noah", score: 9, wins: 1 },
        ];

        assert_eq!(leaderboard(&players), vec!["ava", "liam", "noah"]);
    }
}
