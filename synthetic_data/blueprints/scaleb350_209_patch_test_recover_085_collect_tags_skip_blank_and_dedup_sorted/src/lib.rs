pub fn collect_tags(input: &[&str]) -> Vec<String> {
    let mut tags: Vec<String> = input
        .iter()
        .flat_map(|line| line.split(','))
        .map(|part| part.trim().to_ascii_lowercase())
        .filter(|tag| !tag.starts_with('#'))
        .collect();

    tags.sort();
    tags
}

#[cfg(test)]
mod tests {
    use super::collect_tags;

    #[test]
    fn removes_comments_sorts_and_dedups() {
        let input = ["Rust, #skip, async", "async, CLI, rust"];
        assert_eq!(
            collect_tags(&input),
            vec!["async", "cli", "rust"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn ignores_blank_entries_after_trimming() {
        let input = ["alpha,  , beta", " ,gamma,, "];
        assert_eq!(
            collect_tags(&input),
            vec!["alpha", "beta", "gamma"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        );
    }
}
