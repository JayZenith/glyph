#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn rank_players(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(&b.name))
    });

    let mut out = Vec::new();
    for (i, p) in items.iter().enumerate() {
        out.push(format!("{}. {} ({}, {} wins)", i + 1, p.name, p.score, p.wins));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{rank_players, Player};

    fn p(name: &str, score: u32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            wins,
        }
    }

    #[test]
    fn orders_by_score_then_wins_then_name() {
        let players = vec![
            p("Zoe", 15, 1),
            p("Amy", 20, 2),
            p("Bob", 20, 4),
            p("Cara", 20, 4),
            p("Eli", 15, 3),
        ];

        let ranked = rank_players(&players);
        assert_eq!(
            ranked,
            vec![
                "1. Bob (20, 4 wins)",
                "1. Cara (20, 4 wins)",
                "3. Amy (20, 2 wins)",
                "4. Eli (15, 3 wins)",
                "5. Zoe (15, 1 wins)",
            ]
        );
    }

    #[test]
    fn ties_share_rank_only_on_score_and_wins() {
        let players = vec![
            p("Noah", 9, 2),
            p("Ava", 9, 2),
            p("Mia", 9, 1),
            p("Liam", 8, 9),
        ];

        let ranked = rank_players(&players);
        assert_eq!(
            ranked,
            vec![
                "1. Ava (9, 2 wins)",
                "1. Noah (9, 2 wins)",
                "3. Mia (9, 1 wins)",
                "4. Liam (8, 9 wins)",
            ]
        );
    }
}
