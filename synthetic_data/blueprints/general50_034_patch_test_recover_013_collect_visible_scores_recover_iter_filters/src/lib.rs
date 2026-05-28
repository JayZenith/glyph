pub struct Record<'a> {
    pub name: &'a str,
    pub active: bool,
    pub hidden: bool,
    pub score: Option<i32>,
}

pub fn collect_visible_scores(records: &[Record<'_>], min_score: i32) -> Vec<String> {
    let mut out: Vec<String> = records
        .iter()
        .filter(|r| r.active || !r.hidden)
        .filter_map(|r| {
            let score = r.score.unwrap_or(0);
            if score >= min_score {
                Some(format!("{}:{}", r.name, score))
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
    fn filters_inactive_hidden_and_missing_scores() {
        let records = [
            Record { name: "delta", active: true, hidden: false, score: Some(8) },
            Record { name: "alpha", active: false, hidden: true, score: Some(9) },
            Record { name: "beta", active: true, hidden: true, score: Some(4) },
            Record { name: "gamma", active: true, hidden: false, score: None },
            Record { name: "echo", active: false, hidden: false, score: Some(10) },
        ];

        assert_eq!(collect_visible_scores(&records, 5), vec!["delta:8"]);
    }

    #[test]
    fn trims_names_drops_blank_names_and_sorts_by_score_desc_then_name() {
        let records = [
            Record { name: "  zed  ", active: true, hidden: false, score: Some(7) },
            Record { name: "amy", active: true, hidden: false, score: Some(9) },
            Record { name: "   ", active: true, hidden: false, score: Some(20) },
            Record { name: "bob", active: true, hidden: false, score: Some(9) },
        ];

        assert_eq!(
            collect_visible_scores(&records, 7),
            vec!["amy:9", "bob:9", "zed:7"]
        );
    }
}
