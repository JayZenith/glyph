#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub wins: u32,
    pub losses: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then_with(|| a.losses.cmp(&b.losses))
            .then_with(|| b.name.cmp(&a.name))
    });

    rows.into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} ({}-{})", i + 1, p.name, p.wins, p.losses))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, wins: u32, losses: u32) -> Player {
        Player {
            name: name.to_string(),
            wins,
            losses,
        }
    }

    #[test]
    fn sorts_by_wins_then_fewer_losses_then_name() {
        let players = vec![
            p("zoe", 7, 3),
            p("amy", 9, 4),
            p("bob", 9, 2),
            p("cara", 9, 2),
            p("dina", 7, 1),
        ];

        let got = leaderboard(&players);
        let expected = vec![
            "1. bob (9-2)",
            "2. cara (9-2)",
            "3. amy (9-4)",
            "4. dina (7-1)",
            "5. zoe (7-3)",
        ];

        assert_eq!(got, expected);
    }

    #[test]
    fn keeps_duplicate_records_but_orders_names_alphabetically_on_full_tie() {
        let players = vec![
            p("mila", 4, 4),
            p("ava", 4, 4),
            p("noah", 4, 4),
        ];

        let got = leaderboard(&players);
        let expected = vec![
            "1. ava (4-4)",
            "2. mila (4-4)",
            "3. noah (4-4)",
        ];

        assert_eq!(got, expected);
    }
}
