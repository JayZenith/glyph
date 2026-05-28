#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub score: u8,
    pub active: bool,
}

pub fn parse_records(input: &str) -> Vec<Record> {
    let mut out = Vec::new();

    for line in input.lines() {
        let mut name = String::new();
        let mut score = 0u8;
        let mut active = false;

        for part in line.split(';') {
            if let Some((key, value)) = part.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                match key {
                    "name" => name = value.to_string(),
                    "score" => {
                        if let Ok(v) = value.parse::<u8>() {
                            score = v;
                        }
                    }
                    "active" => active = matches!(value, "yes" | "true"),
                    _ => {}
                }
            }
        }

        if !name.is_empty() {
            out.push(Record { name, score, active });
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_records_missing_required_fields() {
        let input = "name=Alice;score=42;active=yes\nscore=10;active=no\nname=Bob;active=true\nname=;score=9;active=yes";
        assert_eq!(
            parse_records(input),
            vec![Record {
                name: "Alice".into(),
                score: 42,
                active: true,
            }]
        );
    }

    #[test]
    fn rejects_invalid_scores_and_unknown_active_values() {
        let input = "name=Ok;score=100;active=no\nname=TooHigh;score=101;active=yes\nname=BadScore;score=oops;active=true\nname=BadActive;score=7;active=maybe";
        assert_eq!(
            parse_records(input),
            vec![Record {
                name: "Ok".into(),
                score: 100,
                active: false,
            }]
        );
    }
}
