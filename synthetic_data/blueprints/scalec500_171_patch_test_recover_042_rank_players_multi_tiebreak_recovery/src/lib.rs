use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
    pub penalties: u32,
}

pub fn rank_players(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(a.wins.cmp(&b.wins))
            .then(b.penalties.cmp(&a.penalties))
            .then_with(|| a.name.cmp(&b.name))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} [{} pts, {}W, {}P]", i + 1, p.name, p.score, p.wins, p.penalties))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{rank_players, Player};

    fn p(name: &str, score: u32, wins: u32, penalties: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            wins,
            penalties,
        }
    }

    #[test]
    fn ranks_by_score_then_wins_then_lower_penalties() {
        let players = vec![
            p("Delta", 12, 5, 3),
            p("Bravo", 15, 2, 9),
            p("Alpha", 15, 4, 7),
            p("Echo", 15, 4, 2),
        ];

        let ranked = rank_players(&players);
        assert_eq!(
            ranked,
            vec![
                "1. Echo [15 pts, 4W, 2P]",
                "2. Alpha [15 pts, 4W, 7P]",
                "3. Bravo [15 pts, 2W, 9P]",
                "4. Delta [12 pts, 5W, 3P]",
            ]
        );
    }

    #[test]
    fn final_tiebreak_is_case_insensitive_name_order() {
        let players = vec![
            p("zoe", 20, 6, 1),
            p("Amy", 20, 6, 1),
            p("bob", 20, 6, 1),
            p("alice", 20, 6, 1),
        ];

        let ranked = rank_players(&players);
        assert_eq!(
            ranked,
            vec![
                "1. alice [20 pts, 6W, 1P]",
                "2. Amy [20 pts, 6W, 1P]",
                "3. bob [20 pts, 6W, 1P]",
                "4. zoe [20 pts, 6W, 1P]",
            ]
        );
    }
}
