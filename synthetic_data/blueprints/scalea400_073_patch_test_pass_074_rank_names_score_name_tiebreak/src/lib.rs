pub fn ranked_names(entries: &[(&str, u32)]) -> Vec<String> {
    let mut items: Vec<(&str, u32)> = entries.to_vec();
    items.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(b.0)));
    items.into_iter().map(|(name, _)| name.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::ranked_names;

    #[test]
    fn sorts_by_score_descending() {
        let got = ranked_names(&[("amy", 12), ("zoe", 30), ("max", 20)]);
        assert_eq!(got, vec!["zoe", "max", "amy"]);
    }

    #[test]
    fn breaks_ties_by_name_ascending() {
        let got = ranked_names(&[("zoe", 10), ("amy", 10), ("max", 10)]);
        assert_eq!(got, vec!["amy", "max", "zoe"]);
    }

    #[test]
    fn keeps_name_tiebreak_after_higher_scores() {
        let got = ranked_names(&[("zoe", 10), ("bob", 20), ("amy", 20), ("max", 5)]);
        assert_eq!(got, vec!["amy", "bob", "zoe", "max"]);
    }
}
