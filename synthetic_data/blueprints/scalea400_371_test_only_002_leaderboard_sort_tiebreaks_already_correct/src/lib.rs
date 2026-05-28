#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub points: u32,
    pub wins: u32,
    pub penalties: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut ranked = players.to_vec();
    ranked.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| b.wins.cmp(&a.wins))
            .then_with(|| a.penalties.cmp(&b.penalties))
            .then_with(|| a.name.cmp(b.name))
    });

    ranked
        .into_iter()
        .enumerate()
        .map(|(idx, p)| format!("{}. {} ({}/{}/{})", idx + 1, p.name, p.points, p.wins, p.penalties))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{leaderboard, Player};

    #[test]
    fn sorts_by_points_then_wins_then_lower_penalties_then_name() {
        let players = vec![
            Player { name: "Zoe", points: 12, wins: 5, penalties: 4 },
            Player { name: "Amy", points: 12, wins: 5, penalties: 2 },
            Player { name: "Bob", points: 12, wins: 6, penalties: 9 },
            Player { name: "Eli", points: 10, wins: 8, penalties: 0 },
            Player { name: "Ada", points: 12, wins: 5, penalties: 2 },
        ];

        let got = leaderboard(&players);
        let want = vec![
            "1. Bob (12/6/9)",
            "2. Ada (12/5/2)",
            "3. Amy (12/5/2)",
            "4. Zoe (12/5/4)",
            "5. Eli (10/8/0)",
        ];

        assert_eq!(got, want);
    }

    #[test]
    fn keeps_single_entry_format_stable() {
        let players = vec![Player {
            name: "Kai",
            points: 7,
            wins: 3,
            penalties: 1,
        }];

        assert_eq!(leaderboard(&players), vec!["1. Kai (7/3/1)"]);
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let players: Vec<Player> = Vec::new();
        assert!(leaderboard(&players).is_empty());
    }
}
