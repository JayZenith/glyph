pub fn select_codes(entries: &[&str]) -> Vec<String> {
    entries
        .iter()
        .filter_map(|entry| {
            let (code, score) = entry.split_once(':')?;
            let score: i32 = score.parse().ok()?;
            if score >= 50 {
                Some(code.to_ascii_uppercase())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::select_codes;

    #[test]
    fn keeps_only_positive_scores_at_or_above_threshold() {
        let input = ["alpha:75", "beta:49", "gamma:-4", "delta:50"];
        assert_eq!(select_codes(&input), vec!["ALPHA", "DELTA"]);
    }

    #[test]
    fn skips_banned_and_blank_codes() {
        let input = ["skip:90", ":80", "ok:77", "ban:88", "nice:51"];
        assert_eq!(select_codes(&input), vec!["OK", "NICE"]);
    }

    #[test]
    fn trims_parts_and_accepts_zero_padded_scores() {
        let input = ["  mix  :050", "keep:070", " skip :099 "];
        assert_eq!(select_codes(&input), vec!["MIX", "KEEP"]);
    }
}
