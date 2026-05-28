pub fn collect_error_codes(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| {
            let (level, rest) = line.split_once(':')?;
            if level != "ERROR" {
                return None;
            }

            let code = rest
                .split('|')
                .next()
                .unwrap_or("")
                .trim()
                .strip_prefix("code=")?;

            Some(code.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_error_codes;

    #[test]
    fn keeps_only_unique_trimmed_nonzero_error_codes_in_order() {
        let lines = [
            "INFO: code=I1|booted",
            "ERROR: code= E10 |disk full",
            "ERROR: code=0|ignore sentinel",
            "WARN: code=W1|heads up",
            "ERROR: code=E10|duplicate",
            "ERROR: code= |missing",
            "ERROR: code=E20|network",
            "ERROR: message only",
            "ERROR: code=E30 |final",
        ];

        assert_eq!(
            collect_error_codes(&lines),
            vec!["E10", "E20", "E30"]
        );
    }
}
