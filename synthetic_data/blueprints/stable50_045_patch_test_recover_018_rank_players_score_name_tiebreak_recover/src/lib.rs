use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub score: u32,
}

pub fn leaderboard(entries: &[Entry]) -> Vec<(usize, String, u32)> {
    let mut seen = HashSet::new();
    let mut unique = Vec::new();

    for e in entries {
        if seen.insert(e.name.clone()) {
            unique.push(e.clone());
        }
    }

    unique.sort_by(|a, b| b.score.cmp(&a.score));

    let mut out = Vec::new();
    let mut last_score = None;
    let mut rank = 0;

    for (idx, e) in unique.into_iter().enumerate() {
        if last_score != Some(e.score) {
            rank = idx + 1;
            last_score = Some(e.score);
        }
        out.push((rank, e.name, e.score));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(name: &str, score: u32) -> Entry {
        Entry { name: name.to_string(), score }
    }

    #[test]
    fn ranks_by_score_desc_then_name_asc() {
        let rows = vec![e("zoe", 10), e("amy", 10), e("bob", 12), e("cyd", 10)];
        let got = leaderboard(&rows);
        let names: Vec<_> = got.into_iter().map(|(_, n, _)| n).collect();
        assert_eq!(names, vec!["bob", "amy", "cyd", "zoe"]);
    }

    #[test]
    fn uses_dense_ranks_for_tied_scores() {
        let rows = vec![e("bob", 12), e("amy", 10), e("cyd", 10), e("dan", 7)];
        let got = leaderboard(&rows);
        let ranks: Vec<_> = got.into_iter().map(|(r, _, _)| r).collect();
        assert_eq!(ranks, vec![1, 2, 2, 3]);
    }

    #[test]
    fn keeps_first_occurrence_when_name_repeats() {
        let rows = vec![e("amy", 10), e("bob", 12), e("amy", 99), e("cyd", 10)];
        let got = leaderboard(&rows);
        assert_eq!(got, vec![
            (1, "bob".to_string(), 12),
            (2, "amy".to_string(), 10),
            (2, "cyd".to_string(), 10),
        ]);
    }
}
