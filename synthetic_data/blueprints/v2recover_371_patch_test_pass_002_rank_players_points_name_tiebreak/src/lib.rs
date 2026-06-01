#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub points: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| a.wins.cmp(&b.wins))
            .then_with(|| b.name.cmp(&a.name))
    });

    rows.into_iter()
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
        let players = vec![
            p("Zoe", 12, 2),
            p("Amy", 15, 1),
            p("Ben", 15, 3),
            p("Cara", 15, 3),
            p("Dan", 12, 5),
        ];

        let got = leaderboard(&players);
        let want = vec![
            "1. Ben (15/3)",
            "2. Cara (15/3)",
            "3. Amy (15/1)",
            "4. Dan (12/5)",
            "5. Zoe (12/2)",
        ];

        assert_eq!(got, want);
    }

    #[test]
    fn alphabetical_name_breaker_is_ascending() {
        let players = vec![p("Mia", 8, 2), p("Ava", 8, 2), p("Lia", 8, 2)];
        let got = leaderboard(&players);
        let want = vec!["1. Ava (8/2)", "2. Lia (8/2)", "3. Mia (8/2)"];
        assert_eq!(got, want);
    }
}
