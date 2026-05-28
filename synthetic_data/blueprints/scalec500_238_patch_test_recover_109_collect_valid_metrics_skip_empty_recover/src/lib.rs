pub fn collect_metrics(lines: &[&str]) -> Vec<i32> {
    lines
        .iter()
        .filter_map(|line| {
            let (name, value) = line.split_once('=')?;
            let key = name.trim();
            let raw = value.trim();
            if key.starts_with('#') || raw == "skip" {
                return None;
            }
            raw.parse::<i32>().ok().map(|n| n.abs())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_metrics;

    #[test]
    fn keeps_only_non_negative_non_empty_named_metrics() {
        let input = [
            "cpu = 7",
            " = 8",
            "#comment = 3",
            "mem = -5",
            "disk = skip",
            "net = 0",
        ];

        assert_eq!(collect_metrics(&input), vec![7, 0]);
    }

    #[test]
    fn ignores_invalid_and_preserves_order() {
        let input = [
            "temp = 4",
            "broken",
            "fan = x",
            "load = 2",
            "   = 9",
            "qps = -1",
            "users = 5",
        ];

        assert_eq!(collect_metrics(&input), vec![4, 2, 5]);
    }
}
