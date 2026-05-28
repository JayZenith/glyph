#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub age: u32,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let line_no = idx + 1;
        let mut name = None;
        let mut age = None;

        for part in line.split(';') {
            let (key, value) = part
                .split_once('=')
                .ok_or_else(|| format!("line {}: invalid field", line_no))?;

            match key.trim() {
                "name" => name = Some(value.trim().to_string()),
                "age" => {
                    let parsed = value
                        .trim()
                        .parse::<u32>()
                        .map_err(|_| format!("line {}: invalid age", line_no))?;
                    age = Some(parsed);
                }
                _ => return Err(format!("line {}: unknown key", line_no)),
            }
        }

        out.push(Record {
            name: name.ok_or_else(|| format!("line {}: missing name", line_no))?,
            age: age.ok_or_else(|| format!("line {}: missing age", line_no))?,
        });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records() {
        let input = "name=Ann;age=30\nname=Bob;age=8";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    name: "Ann".into(),
                    age: 30,
                },
                Record {
                    name: "Bob".into(),
                    age: 8,
                }
            ]
        );
    }

    #[test]
    fn ignores_blank_and_comment_lines() {
        let input = "# team list\n\nname=Ann;age=30\n   \n# end\nname=Bob;age=8";
        let got = parse_records(input).unwrap();
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].name, "Ann");
        assert_eq!(got[1].name, "Bob");
    }

    #[test]
    fn rejects_unknown_keys() {
        let err = parse_records("name=Ann;role=dev;age=30").unwrap_err();
        assert_eq!(err, "line 1: unknown key");
    }

    #[test]
    fn rejects_negative_age() {
        let err = parse_records("name=Ann;age=-3").unwrap_err();
        assert_eq!(err, "line 1: invalid age");
    }

    #[test]
    fn rejects_missing_fields_on_data_lines() {
        let err = parse_records("name=Ann\nname=Bob;age=8").unwrap_err();
        assert_eq!(err, "line 1: missing age");
    }
}
