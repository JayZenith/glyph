pub fn collect_alerts(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| {
            let (level, message) = line.split_once(':')?;
            let message = message.trim();

            if level == "ERROR" {
                Some(format!("error:{message}"))
            } else if level == "WARN" && !message.is_empty() {
                Some(format!("warn:{message}"))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_alerts;

    #[test]
    fn keeps_errors_and_nonempty_warnings() {
        let input = [
            "INFO:boot",
            "ERROR: disk full ",
            "WARN: cache cold",
            "WARN:   ",
            "DEBUG:trace",
        ];

        assert_eq!(
            collect_alerts(&input),
            vec!["error:disk full", "warn:cache cold"]
        );
    }

    #[test]
    fn ignores_malformed_and_normalizes_error_spacing() {
        let input = [
            "ERROR",
            "ERROR:   bad config   ",
            "WARN:needs attention",
            "WARN",
            "ERROR:   ",
        ];

        assert_eq!(
            collect_alerts(&input),
            vec!["error:bad config", "warn:needs attention"]
        );
    }

    #[test]
    fn treats_lowercase_levels_as_valid_after_normalization() {
        let input = ["error: offline", "warn: delayed", "info:ignored"];

        assert_eq!(
            collect_alerts(&input),
            vec!["error:offline", "warn:delayed"]
        );
    }
}
