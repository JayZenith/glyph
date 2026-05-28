#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub score: u32,
    pub penalty: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedEntry {
    pub rank: usize,
    pub name: String,
    pub score: u32,
    pub penalty: u32,
}

pub fn rank_entries(entries: &[Entry]) -> Vec<RankedEntry> {
    let mut items = entries.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(a.name.cmp(&b.name))
            .then(a.penalty.cmp(&b.penalty))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(i, e)| RankedEntry {
            rank: i + 1,
            name: e.name,
            score: e.score,
            penalty: e.penalty,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(name: &str, score: u32, penalty: u32) -> Entry {
        Entry {
            name: name.to_string(),
            score,
            penalty,
        }
    }

    #[test]
    fn sorts_by_score_then_penalty_then_name() {
        let ranked = rank_entries(&[
            e("zoe", 50, 15),
            e("amy", 50, 10),
            e("bob", 50, 10),
            e("ian", 40, 1),
        ]);

        let names: Vec<_> = ranked.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["amy", "bob", "zoe", "ian"]);
    }

    #[test]
    fn assigns_dense_ranks_for_tied_score_and_penalty() {
        let ranked = rank_entries(&[
            e("bob", 50, 10),
            e("amy", 50, 10),
            e("zoe", 50, 15),
            e("ian", 40, 1),
        ]);

        let pairs: Vec<_> = ranked
            .iter()
            .map(|r| (r.name.as_str(), r.rank))
            .collect();
        assert_eq!(pairs, vec![("amy", 1), ("bob", 1), ("zoe", 2), ("ian", 3)]);
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let ranked = rank_entries(&[]);
        assert!(ranked.is_empty());
    }
}
