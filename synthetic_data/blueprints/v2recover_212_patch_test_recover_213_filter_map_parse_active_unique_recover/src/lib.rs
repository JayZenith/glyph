pub fn collect_enabled_ids(lines: &[&str]) -> Vec<u32> {
    lines
        .iter()
        .filter_map(|line| {
            let mut parts = line.split('|');
            let enabled = parts.next()?;
            let id = parts.next()?;
            if enabled == "on" {
                id.parse::<u32>().ok()
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_enabled_ids;

    #[test]
    fn keeps_only_on_rows_and_valid_positive_ids() {
        let rows = ["on|10", "off|20", "on|x", "on|0", "on|7"];
        assert_eq!(collect_enabled_ids(&rows), vec![10, 7]);
    }

    #[test]
    fn trims_fields_and_skips_duplicates_after_trim() {
        let rows = [" on | 3 ", "on|3", "on| 4", "off|4", "on|4 "];
        assert_eq!(collect_enabled_ids(&rows), vec![3, 4]);
    }

    #[test]
    fn ignores_rows_with_extra_columns_or_missing_parts() {
        let rows = ["on|5|extra", "on|6", "bad", "on|", "|7", "on|8"];
        assert_eq!(collect_enabled_ids(&rows), vec![6, 8]);
    }
}
