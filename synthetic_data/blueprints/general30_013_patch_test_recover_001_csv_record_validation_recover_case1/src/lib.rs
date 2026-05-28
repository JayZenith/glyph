#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub code: String,
    pub qty: u32,
    pub active: bool,
}

pub fn parse_entries(input: &str) -> Result<Vec<Entry>, String> {
    let mut out = Vec::new();

    for (idx, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 3 {
            return Err(format!("line {}: expected 3 fields", idx + 1));
        }

        let code = parts[0].trim().to_string();
        let qty = parts[1].trim().parse::<u32>().map_err(|_| format!("line {}: bad qty", idx + 1))?;
        let active = matches!(parts[2].trim(), "true" | "yes" | "1");

        out.push(Entry { code, qty, active });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_rows() {
        let input = "AB12|7|true\nZX99|0|no";
        let got = parse_entries(input).unwrap();
        assert_eq!(
            got,
            vec![
                Entry { code: "AB12".into(), qty: 7, active: true },
                Entry { code: "ZX99".into(), qty: 0, active: false },
            ]
        );
    }

    #[test]
    fn rejects_extra_fields() {
        let err = parse_entries("AB12|7|true|extra").unwrap_err();
        assert_eq!(err, "line 1: expected 3 fields");
    }

    #[test]
    fn rejects_invalid_code() {
        let err = parse_entries("ab12|7|true").unwrap_err();
        assert_eq!(err, "line 1: bad code");
    }

    #[test]
    fn rejects_unknown_bool() {
        let err = parse_entries("AB12|7|maybe").unwrap_err();
        assert_eq!(err, "line 1: bad active");
    }

    #[test]
    fn trims_and_skips_blank_lines() {
        let input = "  AB12 | 5 | yes \n\n XY00|1|false  ";
        let got = parse_entries(input).unwrap();
        assert_eq!(
            got,
            vec![
                Entry { code: "AB12".into(), qty: 5, active: true },
                Entry { code: "XY00".into(), qty: 1, active: false },
            ]
        );
    }
}
