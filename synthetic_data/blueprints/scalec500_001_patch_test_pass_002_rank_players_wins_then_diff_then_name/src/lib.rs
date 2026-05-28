#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub wins: u32,
    pub losses: u32,
}

impl Player {
    fn diff(&self) -> i32 {
        self.wins as i32 - self.losses as i32
    }
}

pub fn leaderboard(players: &[Player]) -> Vec<&'static str> {
    let mut items: Vec<&Player> = players.iter().collect();
    items.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then_with(|| a.name.cmp(b.name))
    });
    items.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_wins_then_diff_then_name() {
        let players = vec![
            Player { name: "zoe", wins: 8, losses: 5 },
            Player { name: "amy", wins: 8, losses: 2 },
            Player { name: "bob", wins: 10, losses: 8 },
            Player { name: "carl", wins: 10, losses: 1 },
            Player { name: "dina", wins: 8, losses: 2 },
        ];

        assert_eq!(leaderboard(&players), vec!["carl", "bob", "amy", "dina", "zoe"]);
    }

    #[test]
    fn alphabetical_only_after_other_tiebreaks() {
        let players = vec![
            Player { name: "mia", wins: 5, losses: 3 },
            Player { name: "ava", wins: 5, losses: 3 },
            Player { name: "noah", wins: 5, losses: 4 },
        ];

        assert_eq!(leaderboard(&players), vec!["ava", "mia", "noah"]);
    }
}
