use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub wins: u32,
    pub losses: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<&'static str> {
    let mut items: Vec<&Player> = players.iter().collect();
    items.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then_with(|| b.losses.cmp(&a.losses))
            .then_with(|| a.name.cmp(b.name))
    });
    items.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_wins_then_fewer_losses_then_name() {
        let players = vec![
            Player { name: "zoe", wins: 4, losses: 2 },
            Player { name: "amy", wins: 5, losses: 3 },
            Player { name: "max", wins: 5, losses: 1 },
            Player { name: "bob", wins: 5, losses: 1 },
            Player { name: "ivy", wins: 4, losses: 0 },
        ];

        assert_eq!(leaderboard(&players), vec!["bob", "max", "amy", "ivy", "zoe"]);
    }

    #[test]
    fn stable_for_exact_stat_ties_via_name() {
        let players = vec![
            Player { name: "cara", wins: 2, losses: 2 },
            Player { name: "anna", wins: 2, losses: 2 },
            Player { name: "ben", wins: 2, losses: 2 },
        ];

        assert_eq!(leaderboard(&players), vec!["anna", "ben", "cara"]);
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let players: Vec<Player> = vec![];
        assert!(leaderboard(&players).is_empty());
    }
}
