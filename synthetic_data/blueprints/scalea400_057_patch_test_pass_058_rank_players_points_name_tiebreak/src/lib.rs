use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub penalties: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| match b.score.cmp(&a.score) {
        Ordering::Equal => match b.penalties.cmp(&a.penalties) {
            Ordering::Equal => b.name.cmp(&a.name),
            other => other,
        },
        other => other,
    });

    items
        .into_iter()
        .map(|p| format!("{}:{}:{}", p.name, p.score, p.penalties))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32, penalties: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            penalties,
        }
    }

    #[test]
    fn orders_by_score_desc_then_penalties_asc_then_name_asc() {
        let players = vec![
            p("zoe", 20, 1),
            p("amy", 20, 1),
            p("max", 20, 0),
            p("bob", 18, 0),
        ];

        assert_eq!(
            leaderboard(&players),
            vec![
                "max:20:0",
                "amy:20:1",
                "zoe:20:1",
                "bob:18:0",
            ]
        );
    }

    #[test]
    fn keeps_duplicate_scores_with_stable_tiebreak_rules() {
        let players = vec![
            p("cara", 15, 2),
            p("ben", 15, 2),
            p("anna", 15, 1),
            p("dan", 16, 5),
        ];

        assert_eq!(
            leaderboard(&players),
            vec![
                "dan:16:5",
                "anna:15:1",
                "ben:15:2",
                "cara:15:2",
            ]
        );
    }
}
