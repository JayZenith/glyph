use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub solved: u32,
}

pub fn leaderboard(mut players: Vec<Player>) -> Vec<String> {
    players.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| b.solved.cmp(&a.solved))
            .then_with(|| b.name.cmp(&a.name))
    });

    players
        .into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} [{}|{}]", i + 1, p.name, p.score, p.solved))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32, solved: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            solved,
        }
    }

    #[test]
    fn sorts_by_score_then_solved_then_name() {
        let rows = leaderboard(vec![
            p("Zoe", 120, 5),
            p("Amy", 150, 3),
            p("Bob", 150, 4),
            p("Ada", 150, 4),
            p("Cid", 120, 8),
        ]);

        assert_eq!(
            rows,
            vec![
                "1. Ada [150|4]",
                "2. Bob [150|4]",
                "3. Amy [150|3]",
                "4. Cid [120|8]",
                "5. Zoe [120|5]",
            ]
        );
    }

    #[test]
    fn equal_metrics_use_alphabetical_name_tiebreak() {
        let rows = leaderboard(vec![
            p("mila", 90, 2),
            p("anna", 90, 2),
            p("zoe", 90, 2),
        ]);

        assert_eq!(
            rows,
            vec![
                "1. anna [90|2]",
                "2. mila [90|2]",
                "3. zoe [90|2]",
            ]
        );
    }
}
