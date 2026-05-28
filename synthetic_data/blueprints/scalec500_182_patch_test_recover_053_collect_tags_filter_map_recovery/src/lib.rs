pub fn collect_tags(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .filter_map(|item| item.strip_prefix("tag:"))
        .filter(|tag| !tag.is_empty())
        .map(|tag| tag.to_ascii_lowercase())
        .fold(Vec::new(), |mut acc, tag| {
            if !acc.contains(&tag) {
                acc.push(tag);
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::collect_tags;

    #[test]
    fn keeps_first_occurrence_order_case_insensitive() {
        let input = ["tag:Rust", "skip", "tag:rust", "tag:CLI", "tag:cli", "tag:web"];
        assert_eq!(collect_tags(&input), vec!["rust", "cli", "web"]);
    }

    #[test]
    fn trims_and_skips_blank_tags() {
        let input = ["tag:  ops  ", "tag:   ", "tag:dev"];
        assert_eq!(collect_tags(&input), vec!["ops", "dev"]);
    }

    #[test]
    fn ignores_prefix_with_wrong_case() {
        let input = ["Tag:nope", "tag:yes", "TAG:also-no"];
        assert_eq!(collect_tags(&input), vec!["yes"]);
    }
}
