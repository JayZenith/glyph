use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: i32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut best_by_name: HashMap<String, Player> = HashMap::new();
    for p in players {
        best_by_name
            .entry(p.name.clone())
            .and_modify(|cur| {
                if p.score > cur.score {
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
    });

    rows.into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} ({} pts, {} wins)", i + 1, p.name, p.score, p.wins))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: i32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            wins,
        }
    }

    #[test]
    fn sorts_by_score_then_wins_then_name() {
        let board = leaderboard(&[
            p("Nia", 12, 3),
            p("Ava", 12, 5),
            p("Moe", 12, 5),
            p("Zed", 18, 1),
        ]);

        assert_eq!(
            board,
            vec![
                "1. Zed (18 pts, 1 wins)",
                "2. Ava (12 pts, 5 wins)",
                "2. Moe (12 pts, 5 wins)",
                "4. Nia (12 pts, 3 wins)",
            ]
        );
    }

    #[test]
    fn duplicate_names_keep_better_record_before_ranking() {
        let board = leaderboard(&[
            p("Kai", 10, 3),
            p("Kai", 10, 5),
            p("Bea", 11, 1),
            p("Lia", 10, 5),
        ]);

        assert_eq!(
            board,
            vec![
                "1. Bea (11 pts, 1 wins)",
                "2. Kai (10 pts, 5 wins)",
                "2. Lia (10 pts, 5 wins)",
            ]
        );
    }

    #[test]
    fn duplicate_with_higher_score_still_wins_even_if_fewer_wins() {
        let board = leaderboard(&[
            p("Ivy", 8, 7),
            p("Ivy", 9, 1),
            p("Omar", 9, 0),
        ]);

        assert_eq!(
            board,
            vec![
                "1. Ivy (9 pts, 1 wins)",
                "2. Omar (9 pts, 0 wins)",
            ]
        );
    }
}
