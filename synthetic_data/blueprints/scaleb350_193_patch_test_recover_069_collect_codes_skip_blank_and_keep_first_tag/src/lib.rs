pub fn collect_codes(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| {
            let mut parts = line.split('|');
            let code = parts.next()?.trim();
            let tag = parts.next()?.trim();

            if tag.eq_ignore_ascii_case("skip") {
                None
            } else {
                Some(format!("{}:{}", code.to_uppercase(), tag.to_lowercase()))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_codes;

    #[test]
    fn skips_blank_codes_and_skip_tag() {
        let input = [" ab1 | keep ", "   | keep", "cd2|SKIP", " ef3 | go "];
        let got = collect_codes(&input);
        assert_eq!(got, vec!["AB1:keep", "EF3:go"]);
    }

    #[test]
    fn ignores_extra_segments_after_first_tag() {
        let input = ["x1|Keep|unused", "y2|go|later", "z3|skip|ignored"];
        let got = collect_codes(&input);
        assert_eq!(got, vec!["X1:keep", "Y2:go"]);
    }

    #[test]
    fn requires_a_non_empty_tag() {
        let input = ["a1|", "b2|   ", "c3|ok"];
        let got = collect_codes(&input);
        assert_eq!(got, vec!["C3:ok"]);
    }
}
