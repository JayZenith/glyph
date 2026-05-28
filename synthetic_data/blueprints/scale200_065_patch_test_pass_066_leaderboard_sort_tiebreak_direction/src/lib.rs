#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(a.wins.cmp(&b.wins))
            .then(a.name.cmp(&b.name))
    });

    items
        .into_iter()
        .map(|p| format!("{}:{}:{}", p.name, p.score, p.wins))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{leaderboard, Player};

    fn p(name: &str, score: u32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            wins,
        }
    }

    #[test]
    fn sorts_by_score_then_wins_then_name() {
        let players = vec![
            p("zoe", 20, 2),
            p("anna", 20, 5),
            p("mike", 20, 5),
            p("beth", 25, 1),
        ];

        assert_eq!(
            leaderboard(&players),
            vec![
                "beth:25:1",
                "anna:20:5",
                "mike:20:5",
                "zoe:20:2",
            ]
        );
    }

    #[test]
    fn keeps_alphabetical_order_when_score_and_wins_tie() {
        let players = vec![p("liam", 9, 3), p("emma", 9, 3), p("noah", 9, 3)];

        assert_eq!(
            leaderboard(&players),
            vec!["emma:9:3", "liam:9:3", "noah:9:3"]
        );
    }
}
