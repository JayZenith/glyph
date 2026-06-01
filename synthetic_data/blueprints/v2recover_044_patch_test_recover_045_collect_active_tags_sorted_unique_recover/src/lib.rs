use std::collections::HashSet;

#[derive(Debug)]
pub struct Record<'a> {
    pub active: bool,
    pub tags: &'a [&'a str],
}

pub fn collect_tags(records: &[Record<'_>]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut out: Vec<String> = records
        .iter()
        .flat_map(|record| record.tags.iter())
        .filter_map(|tag| {
            let trimmed = tag.trim();
            if trimmed.is_empty() {
                None
            } else if seen.insert(trimmed.to_string()) {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect();
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_active_trimmed_unique_case_insensitive_sorted() {
        let records = [
            Record {
                active: true,
                tags: &["  beta", "Alpha", "", "alpha  "],
            },
            Record {
                active: false,
                tags: &["gamma", "Beta"],
            },
            Record {
                active: true,
                tags: &["delta", "  ", "ALPHA", "beta"],
            },
        ];

        assert_eq!(collect_tags(&records), vec!["Alpha", "beta", "delta"]);
    }

    #[test]
    fn preserves_first_trimmed_spelling_for_duplicates() {
        let records = [
            Record {
                active: true,
                tags: &["  Rust  ", "rust", "RUST", "cargo"],
            },
            Record {
                active: true,
                tags: &[" Cargo ", "rust ", ""],
            },
        ];

        assert_eq!(collect_tags(&records), vec!["cargo", "Rust"]);
    }
}
