pub fn extract_tags(input: &str) -> Vec<String> {
    input
        .lines()
        .filter_map(|line| line.strip_prefix("tag:"))
        .map(|rest| rest.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::extract_tags;

    #[test]
    fn collects_trimmed_tag_values() {
        let input = "tag: apple\nnoop\ntag: banana ";
        assert_eq!(extract_tags(input), vec!["apple", "banana"]);
    }

    #[test]
    fn ignores_empty_tag_entries() {
        let input = "tag:   \ntag: pear\ntag:";
        assert_eq!(extract_tags(input), vec!["pear"]);
    }

    #[test]
    fn ignores_comment_tag_entries() {
        let input = "tag:#hidden\ntag: kiwi\ntag:   #also hidden\ntag:melon";
        assert_eq!(extract_tags(input), vec!["kiwi", "melon"]);
    }
}
