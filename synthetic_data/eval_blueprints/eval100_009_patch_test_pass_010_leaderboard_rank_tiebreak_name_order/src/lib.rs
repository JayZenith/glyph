#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub solved: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.solved.cmp(&b.solved))
            .then_with(|| b.name.cmp(&a.name))
    });

    rows.into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} [{} pts, {} solved]", i + 1, p.name, p.score, p.solved))
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
    fn sorts_by_score_then_fewer_solved_then_name() {
        let players = vec![
            p("Zoe", 120, 5),
            p("Amy", 120, 5),
            p("Bob", 120, 4),
            p("Cara", 90, 3),
        ];

        let got = leaderboard(&players);
        let want = vec![
            "1. Bob [120 pts, 4 solved]",
            "2. Amy [120 pts, 5 solved]",
            "3. Zoe [120 pts, 5 solved]",
            "4. Cara [90 pts, 3 solved]",
        ];

        assert_eq!(got, want);
    }

    #[test]
    fn empty_input_returns_no_rows() {
        let got = leaderboard(&[]);
        assert!(got.is_empty());
    }

    #[test]
    fn numbering_follows_sorted_order() {
        let players = vec![p("Nia", 50, 2), p("Ian", 75, 6), p("Omar", 75, 4)];

        let got = leaderboard(&players);
        assert_eq!(got[0], "1. Omar [75 pts, 4 solved]");
        assert_eq!(got[1], "2. Ian [75 pts, 6 solved]");
        assert_eq!(got[2], "3. Nia [50 pts, 2 solved]");
    }
}
