pub fn collect_tagged_values(lines: &[&str], tag: &str) -> Vec<i32> {
    lines
        .iter()
        .filter_map(|line| {
            let (prefix, value) = line.split_once(':')?;
            if prefix != tag {
                return None;
            }
            value.trim().parse::<i32>().ok()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_tagged_values;

    #[test]
    fn keeps_only_matching_tagged_numbers() {
        let lines = ["keep:1", "skip:9", "keep:2", "other:3"];
        assert_eq!(collect_tagged_values(&lines, "keep"), vec![1, 2]);
    }

    #[test]
    fn trims_values_and_skips_invalid_or_empty_tag_prefixes() {
        let lines = ["keep: 7", ":11", "keep:", "keep:bad", "keep: 8 "];
        assert_eq!(collect_tagged_values(&lines, "keep"), vec![7, 8]);
    }

    #[test]
    fn trims_tag_before_matching() {
        let lines = [" keep :4", "keep:5", " keep: 6", "skip:7"];
        assert_eq!(collect_tagged_values(&lines, "keep"), vec![4, 5, 6]);
    }
}
