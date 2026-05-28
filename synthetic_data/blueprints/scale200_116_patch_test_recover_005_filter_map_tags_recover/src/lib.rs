pub fn selected_labels(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .filter_map(|line| {
            let (name, flag) = line.split_once('|')?;
            if flag != "keep" {
                return None;
            }
            Some(name.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::selected_labels;

    #[test]
    fn keeps_only_marked_items() {
        let items = ["alpha|keep", "beta|drop", "gamma|keep"];
        assert_eq!(selected_labels(&items), vec!["alpha", "gamma"]);
    }

    #[test]
    fn trims_names_and_skips_blank_names() {
        let items = ["  alpha  |keep", "   |keep", "beta|keep"];
        assert_eq!(selected_labels(&items), vec!["alpha", "beta"]);
    }

    #[test]
    fn keep_flag_is_case_insensitive() {
        let items = ["one|KEEP", "two|Keep", "three|drop"];
        assert_eq!(selected_labels(&items), vec!["one", "two"]);
    }
}
