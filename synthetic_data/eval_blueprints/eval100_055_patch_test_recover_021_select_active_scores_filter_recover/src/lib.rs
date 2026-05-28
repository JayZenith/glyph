pub struct Entry<'a> {
    pub id: &'a str,
    pub active: bool,
    pub score: Option<i32>,
}

pub fn select_ids(entries: &[Entry<'_>], min_score: i32) -> Vec<String> {
    entries
        .iter()
        .filter(|e| e.active || e.score.unwrap_or(0) >= min_score)
        .map(|e| (e.id, e.score.unwrap_or(0)))
        .filter(|(_, score)| *score >= min_score)
        .map(|(id, _)| id.to_ascii_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{select_ids, Entry};

    #[test]
    fn keeps_only_active_entries_with_present_score_at_or_above_min() {
        let entries = [
            Entry { id: "alpha", active: true, score: Some(8) },
            Entry { id: "beta", active: false, score: Some(12) },
            Entry { id: "gamma", active: true, score: None },
            Entry { id: "delta", active: true, score: Some(5) },
            Entry { id: "epsilon", active: true, score: Some(10) },
        ];

        assert_eq!(select_ids(&entries, 8), vec!["ALPHA", "EPSILON"]);
    }

    #[test]
    fn drops_blank_ids_and_preserves_input_order() {
        let entries = [
            Entry { id: "ok", active: true, score: Some(3) },
            Entry { id: "", active: true, score: Some(9) },
            Entry { id: "later", active: true, score: Some(4) },
            Entry { id: "skip", active: false, score: Some(99) },
        ];

        assert_eq!(select_ids(&entries, 3), vec!["OK", "LATER"]);
    }

    #[test]
    fn exact_threshold_is_included() {
        let entries = [
            Entry { id: "edge", active: true, score: Some(7) },
            Entry { id: "low", active: true, score: Some(6) },
        ];

        assert_eq!(select_ids(&entries, 7), vec!["EDGE"]);
    }
}
