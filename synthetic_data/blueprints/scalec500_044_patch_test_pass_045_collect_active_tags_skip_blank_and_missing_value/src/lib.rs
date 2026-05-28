pub fn collect_active_tags(rows: &[&str]) -> Vec<String> {
    rows
        .iter()
        .filter_map(|row| {
            let mut parts = row.split('|');
            let status = parts.next()?;
            let key = parts.next()?;
            let value = parts.next()?;

            if status == "active" && key == "tag" {
                Some(value.trim().to_string())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_active_tags;

    #[test]
    fn keeps_only_active_tag_values() {
        let rows = [
            "active|tag|alpha",
            "inactive|tag|beta",
            "active|note|ignore",
            "active|tag|gamma",
        ];

        assert_eq!(collect_active_tags(&rows), vec!["alpha", "gamma"]);
    }

    #[test]
    fn skips_blank_values_and_missing_third_field() {
        let rows = [
            "active|tag|  ",
            "active|tag",
            "active|tag|delta",
            "active|tag|  echo  ",
        ];

        assert_eq!(collect_active_tags(&rows), vec!["delta", "echo"]);
    }

    #[test]
    fn preserves_input_order_after_filtering() {
        let rows = [
            "inactive|tag|skip",
            "active|tag|first",
            "active|tag|second",
            "active|note|skip",
            "active|tag|third",
        ];

        assert_eq!(collect_active_tags(&rows), vec!["first", "second", "third"]);
    }
}
