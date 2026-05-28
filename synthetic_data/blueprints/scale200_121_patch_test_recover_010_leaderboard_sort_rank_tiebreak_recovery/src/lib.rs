use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub solved: u32,
    pub penalty: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: String,
    pub score: u32,
    pub solved: u32,
    pub penalty: u32,
}

fn cmp_player(a: &Player, b: &Player) -> Ordering {
    b.score
        .cmp(&a.score)
        .then_with(|| a.penalty.cmp(&b.penalty))
        .then_with(|| a.name.cmp(&b.name))
}

pub fn leaderboard(players: &[Player]) -> Vec<RankedPlayer> {
    let mut ordered = players.to_vec();
    ordered.sort_by(cmp_player);

    ordered
        .into_iter()
        .enumerate()
        .map(|(i, p)| RankedPlayer {
            rank: i + 1,
            name: p.name,
            score: p.score,
            solved: p.solved,
            penalty: p.penalty,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32, solved: u32, penalty: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            solved,
            penalty,
        }
    }

    #[test]
    fn sorts_by_score_then_solved_then_penalty_then_name() {
        let players = vec![
            p("zoe", 120, 4, 10),
            p("amy", 120, 5, 30),
            p("bob", 120, 5, 20),
            p("cal", 120, 5, 20),
            p("dan", 110, 8, 5),
        ];

        let board = leaderboard(&players);
        let names: Vec<_> = board.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["bob", "cal", "amy", "zoe", "dan"]);
    }

    #[test]
    fn ties_share_rank_and_later_rank_skips() {
        let players = vec![
            p("amy", 100, 4, 10),
            p("bob", 100, 4, 10),
            p("cal", 95, 6, 3),
            p("dan", 95, 6, 3),
            p("eve", 90, 7, 1),
        ];

        let board = leaderboard(&players);
        let pairs: Vec<_> = board.iter().map(|r| (r.name.as_str(), r.rank)).collect();
        assert_eq!(pairs, vec![("amy", 1), ("bob", 1), ("cal", 3), ("dan", 3), ("eve", 5)]);
    }

    #[test]
    fn identical_metrics_use_name_only_for_order_not_rank() {
        let players = vec![
            p("mila", 70, 2, 9),
            p("ava", 70, 2, 9),
            p("noah", 70, 2, 9),
        ];

        let board = leaderboard(&players);
        let names: Vec<_> = board.iter().map(|r| r.name.as_str()).collect();
        let ranks: Vec<_> = board.iter().map(|r| r.rank).collect();
        assert_eq!(names, vec!["ava", "mila", "noah"]);
        assert_eq!(ranks, vec![1, 1, 1]);
    }
}
