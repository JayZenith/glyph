use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: &'static str,
    pub solved: u32,
    pub penalty: u32,
    pub last_submit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ranked {
    pub rank: usize,
    pub name: &'static str,
    pub solved: u32,
    pub penalty: u32,
    pub last_submit: u32,
}

pub fn leaderboard(entries: &[Entry]) -> Vec<Ranked> {
    let mut items = entries.to_vec();
    items.sort_by(|a, b| {
        b.solved.cmp(&a.solved)
            .then_with(|| a.penalty.cmp(&b.penalty))
            .then_with(|| a.name.cmp(&b.name))
    });

    let mut out = Vec::with_capacity(items.len());
    let mut prev: Option<&Entry> = None;
    let mut rank = 1;

    for (i, e) in items.iter().enumerate() {
        if let Some(p) = prev {
            if p.solved != e.solved || p.penalty != e.penalty {
                rank = i + 1;
            }
        }
        out.push(Ranked {
            rank,
            name: e.name,
            solved: e.solved,
            penalty: e.penalty,
            last_submit: e.last_submit,
        });
        prev = Some(e);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_all_tiebreakers() {
        let rows = vec![
            Entry { name: "delta", solved: 4, penalty: 300, last_submit: 80 },
            Entry { name: "bravo", solved: 4, penalty: 300, last_submit: 70 },
            Entry { name: "charlie", solved: 4, penalty: 250, last_submit: 90 },
            Entry { name: "alpha", solved: 5, penalty: 500, last_submit: 120 },
        ];
        let got = leaderboard(&rows);
        let names: Vec<_> = got.iter().map(|r| r.name).collect();
        assert_eq!(names, vec!["alpha", "charlie", "bravo", "delta"]);
        let ranks: Vec<_> = got.iter().map(|r| r.rank).collect();
        assert_eq!(ranks, vec![1, 2, 3, 4]);
    }

    #[test]
    fn equal_score_penalty_and_time_share_rank() {
        let rows = vec![
            Entry { name: "zoe", solved: 3, penalty: 400, last_submit: 100 },
            Entry { name: "amy", solved: 3, penalty: 400, last_submit: 100 },
            Entry { name: "max", solved: 2, penalty: 200, last_submit: 50 },
        ];
        let got = leaderboard(&rows);
        let names: Vec<_> = got.iter().map(|r| r.name).collect();
        assert_eq!(names, vec!["amy", "zoe", "max"]);
        let ranks: Vec<_> = got.iter().map(|r| r.rank).collect();
        assert_eq!(ranks, vec![1, 1, 3]);
    }
}
