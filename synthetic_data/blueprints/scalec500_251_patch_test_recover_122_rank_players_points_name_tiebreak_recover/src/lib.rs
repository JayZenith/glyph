#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub points: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<&'static str> {
    let mut items: Vec<&Player> = players.iter().collect();
    items.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.wins.cmp(&b.wins))
            .then(a.name.cmp(b.name))
    });
    items.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_points_then_wins_then_name() {
        let players = vec![
            Player { name: "zoe", points: 12, wins: 4 },
            Player { name: "amy", points: 15, wins: 2 },
            Player { name: "bob", points: 15, wins: 5 },
            Player { name: "ada", points: 15, wins: 5 },
            Player { name: "max", points: 9, wins: 9 },
        ];

        assert_eq!(leaderboard(&players), vec!["ada", "bob", "amy", "zoe", "max"]);
    }

    #[test]
    fn alphabetical_tiebreak_for_full_tie_on_scores() {
        let players = vec![
            Player { name: "mia", points: 7, wins: 3 },
            Player { name: "ava", points: 7, wins: 3 },
            Player { name: "ivy", points: 7, wins: 3 },
        ];

        assert_eq!(leaderboard(&players), vec!["ava", "ivy", "mia"]);
    }
}
