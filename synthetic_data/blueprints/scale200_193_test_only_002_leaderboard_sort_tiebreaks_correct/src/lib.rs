#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub points: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| b.wins.cmp(&a.wins))
            .then_with(|| a.name.cmp(b.name))
    });

    rows.into_iter()
        .enumerate()
        .map(|(idx, p)| format!("{}. {} ({}, {} wins)", idx + 1, p.name, p.points, p.wins))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_points_then_wins_then_name() {
        let players = vec![
            Player {
                name: "Mina",
                points: 12,
                wins: 4,
            },
            Player {
                name: "Omar",
                points: 15,
                wins: 3,
            },
            Player {
                name: "Ava",
                points: 15,
                wins: 5,
            },
            Player {
                name: "Zed",
                points: 15,
                wins: 5,
            },
        ];

        let ranked = leaderboard(&players);
        assert_eq!(
            ranked,
            vec![
                "1. Ava (15, 5 wins)",
                "2. Zed (15, 5 wins)",
                "3. Omar (15, 3 wins)",
                "4. Mina (12, 4 wins)",
            ]
        );
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let players = Vec::new();
        let ranked = leaderboard(&players);
        assert!(ranked.is_empty());
    }

    #[test]
    fn stable_formatting_for_single_player() {
        let players = vec![Player {
            name: "Kai",
            points: 7,
            wins: 2,
        }];

        let ranked = leaderboard(&players);
        assert_eq!(ranked, vec!["1. Kai (7, 2 wins)"]);
    }
}
