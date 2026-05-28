pub fn passing_names(entries: &[(&str, Option<u32>)], min_score: u32) -> Vec<String> {
    entries
        .iter()
        .filter_map(|(name, score)| score.map(|s| (*name, s)))
        .filter(|(_, score)| *score >= min_score)
        .map(|(name, _)| name.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::passing_names;

    #[test]
    fn skips_missing_and_low_scores() {
        let data = [
            ("Ada", Some(91)),
            ("Bob", None),
            ("Cy", Some(70)),
            ("Di", Some(88)),
        ];

        assert_eq!(passing_names(&data, 80), vec!["Ada", "Di"]);
    }

    #[test]
    fn keeps_input_order_for_matches() {
        let data = [
            ("Ivy", Some(50)),
            ("Jae", Some(83)),
            ("Kai", Some(83)),
            ("Lux", None),
        ];

        assert_eq!(passing_names(&data, 83), vec!["Jae", "Kai"]);
    }

    #[test]
    fn empty_when_nothing_qualifies() {
        let data = [("Moe", None), ("Nia", Some(10))];
        let got: Vec<String> = passing_names(&data, 20);
        assert!(got.is_empty());
    }
}
