pub fn select_tags(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .filter_map(|item| {
            let word = item.trim();
            if word.len() % 2 == 1 {
                Some(word.to_uppercase())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::select_tags;

    #[test]
    fn keeps_even_length_words_uppercased() {
        let input = [" pear ", "fig", "plum", "kiwi", "a", "melon"];
        assert_eq!(
            select_tags(&input),
            vec!["PEAR".to_string(), "PLUM".to_string(), "KIWI".to_string()]
        );
    }

    #[test]
    fn drops_empty_and_odd_length_words() {
        let input = ["", "  ", "one", "three", " seven "];
        let actual = select_tags(&input);
        assert!(actual.is_empty());
    }
}
