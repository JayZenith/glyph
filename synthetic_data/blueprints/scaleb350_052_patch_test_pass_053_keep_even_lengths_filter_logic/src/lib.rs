pub fn even_length_uppercase(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .filter(|w| w.len() % 2 == 1)
        .map(|w| w.to_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::even_length_uppercase;

    #[test]
    fn keeps_only_even_lengths_in_order() {
        let input = ["a", "to", "cat", "door", "trees", "ship"];
        let got = even_length_uppercase(&input);
        assert_eq!(got, vec!["TO", "DOOR", "SHIP"]);
    }

    #[test]
    fn skips_empty_result_when_no_even_lengths() {
        let input = ["a", "bee", "wow"];
        let got = even_length_uppercase(&input);
        assert!(got.is_empty());
    }
}
