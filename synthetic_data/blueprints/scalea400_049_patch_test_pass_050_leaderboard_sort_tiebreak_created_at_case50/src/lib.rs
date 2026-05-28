#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub points: u32,
    pub wins: u32,
    pub created_at: u32,
}

pub fn leaderboard(mut players: Vec<Player>) -> Vec<&'static str> {
    players.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| b.wins.cmp(&a.wins))
            .then_with(|| b.created_at.cmp(&a.created_at))
            .then_with(|| a.name.cmp(b.name))
    });

    players.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_by_points_then_wins_then_older_signup_then_name() {
        let players = vec![
            Player { name: "zoe", points: 12, wins: 4, created_at: 30 },
            Player { name: "amy", points: 12, wins: 4, created_at: 10 },
            Player { name: "bob", points: 12, wins: 5, created_at: 20 },
            Player { name: "ian", points: 9, wins: 9, created_at: 1 },
        ];

        assert_eq!(leaderboard(players), vec!["bob", "amy", "zoe", "ian"]);
    }

    #[test]
    fn final_tiebreak_is_name_ascending() {
        let players = vec![
            Player { name: "mila", points: 7, wins: 2, created_at: 8 },
            Player { name: "ava", points: 7, wins: 2, created_at: 8 },
            Player { name: "noah", points: 7, wins: 2, created_at: 8 },
        ];

        assert_eq!(leaderboard(players), vec!["ava", "mila", "noah"]);
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let players = Vec::new();
        let ranked: Vec<&'static str> = leaderboard(players);
        assert!(ranked.is_empty());
    }
}
