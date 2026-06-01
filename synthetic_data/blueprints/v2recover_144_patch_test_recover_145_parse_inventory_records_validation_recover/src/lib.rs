#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub sku: String,
    pub qty: u32,
    pub active: bool,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();

    for (idx, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 {
            return Err(format!("line {}: expected 3 fields", idx + 1));
        }

        let sku = parts[0].to_string();
        if sku.is_empty() {
            return Err(format!("line {}: empty sku", idx + 1));
        }

        let qty: u32 = parts[1]
            .parse()
            .map_err(|_| format!("line {}: invalid qty", idx + 1))?;

        let active = match parts[2] {
            "Y" => true,
            "N" => false,
            _ => return Err(format!("line {}: invalid active flag", idx + 1)),
        };

        out.push(Record { sku, qty, active });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records_and_ignores_comment_lines() {
        let input = "# header\nA12|5|Y\n\nB-7|0|N\n";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    sku: "A12".into(),
                    qty: 5,
                    active: true,
                },
                Record {
                    sku: "B-7".into(),
                    qty: 0,
                    active: false,
                }
            ]
        );
    }

    #[test]
    fn trims_fields_and_accepts_lowercase_active_flag() {
        let input = "  ZX-9 | 42 | y ";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![Record {
                sku: "ZX-9".into(),
                qty: 42,
                active: true,
            }]
        );
    }

    #[test]
    fn rejects_bad_sku_characters() {
        let err = parse_records("BAD SKU|3|N").unwrap_err();
        assert_eq!(err, "line 1: invalid sku");
    }

    #[test]
    fn rejects_negative_qty() {
        let err = parse_records("A1|-3|Y").unwrap_err();
        assert_eq!(err, "line 1: invalid qty");
    }
}
