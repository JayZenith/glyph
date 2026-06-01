#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub points: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.wins.cmp(&b.wins))
            .then(a.name.cmp(&b.name))
    });
    items
        .into_iter()
        .enumerate()
        .map(|(idx, p)| format!("{}. {} ({}/{})", idx + 1, p.name, p.points, p.wins))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, points: u32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            points,
            wins,
        }
    }

    #[test]
    fn sorts_by_points_then_wins_then_name() {
        let rows = leaderboard(&[
            p("Mia", 12, 5),
            p("Ava", 14, 3),
            p("Zoe", 14, 4),
            p("Bea", 14, 4),
            p("Lia", 12, 6),
        ]);

        assert_eq!(
            rows,
            vec![
                "1. Bea (14/4)",
                "2. Zoe (14/4)",
                "3. Ava (14/3)",
                "4. Lia (12/6)",
                "5. Mia (12/5)",
            ]
        );
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let rows = leaderboard(&[]);
        assert!(rows.is_empty());
    }

    #[test]
    fn name_tiebreak_is_ascending_only_after_other_descending_rules() {
        let rows = leaderboard(&[
            p("Cara", 9, 2),
            p("Anna", 9, 2),
            p("Bella", 9, 3),
        ]);

        assert_eq!(
            rows,
            vec!["1. Bella (9/3)", "2. Anna (9/2)", "3. Cara (9/2)"]
        );
    }
}
