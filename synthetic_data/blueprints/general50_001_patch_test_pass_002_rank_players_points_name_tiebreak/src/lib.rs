use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut best_by_name: HashMap<String, Player> = HashMap::new();

    for p in players {
        best_by_name
            .entry(p.name.clone())
            .and_modify(|cur| {
                if p.score >= cur.score {
                    *cur = p.clone();
                }
            })
            .or_insert_with(|| p.clone());
    }

    let mut rows: Vec<Player> = best_by_name.into_values().collect();
    rows.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| b.wins.cmp(&a.wins))
    });

    rows.into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} ({}, {} wins)", i + 1, p.name, p.score, p.wins))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            wins,
        }
    }

    #[test]
    fn ranks_by_score_then_wins_then_name() {
        let players = vec![
            p("Zoe", 10, 1),
            p("Amy", 10, 3),
            p("Bob", 10, 3),
            p("Cara", 12, 0),
        ];

        assert_eq!(
            leaderboard(&players),
            vec![
                "1. Cara (12, 0 wins)",
                "2. Amy (10, 3 wins)",
                "3. Bob (10, 3 wins)",
                "4. Zoe (10, 1 wins)",
            ]
        );
    }

    #[test]
    fn duplicate_names_keep_best_score_then_best_wins() {
        let players = vec![
            p("Mia", 8, 4),
            p("Mia", 10, 1),
            p("Mia", 10, 3),
            p("Noah", 9, 5),
        ];

        assert_eq!(
            leaderboard(&players),
            vec![
                "1. Mia (10, 3 wins)",
                "2. Noah (9, 5 wins)",
            ]
        );
    }
}
