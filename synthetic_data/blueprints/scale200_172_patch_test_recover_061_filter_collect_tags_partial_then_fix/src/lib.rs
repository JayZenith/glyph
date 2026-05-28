pub fn collect_tags(lines: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = lines
        .iter()
        .filter_map(|line| {
            let (user, rest) = line.split_once(':')?;
            if user.is_empty() {
                return None;
            }

            let tag = rest.split(',').next()?.trim();
            if tag.is_empty() {
                return None;
            }

            Some(tag.to_ascii_lowercase())
        })
        .collect();

    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::collect_tags;

    #[test]
    fn keeps_active_non_blocked_first_tags_sorted_unique() {
        let lines = [
            "alice: Rust,cli ; active",
            "bob: ops,infra ; inactive",
            "cara: rust,ffi ; active",
            "dave: misc ; active blocked",
            "erin:  Data ,etl ; active",
            "fran: ; active",
            "gina no colon",
            ": orphan ; active",
            "hank: Rust ,embedded ; active",
        ];

        assert_eq!(collect_tags(&lines), vec!["data", "rust"]);
    }

    #[test]
    fn trims_first_tag_and_ignores_case_duplicates() {
        let lines = [
            "u1:  Alpha  ,x ; active",
            "u2: alpha,z ; active",
            "u3: beta ; active blocked",
            "u4: gamma ; inactive",
            "u5: Beta ; active",
        ];

        assert_eq!(collect_tags(&lines), vec!["alpha", "beta"]);
    }
}
