use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub port: u16,
    pub enabled: bool,
}

pub fn parse_entry(input: &str) -> Result<Entry, String> {
    let mut map = HashMap::new();

    for raw_line in input.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("invalid line: {line}"))?;
        map.insert(key.trim(), value.trim());
    }

    let name = map.get("name").ok_or("missing name")?.to_string();
    let port = map
        .get("port")
        .ok_or("missing port")?
        .parse::<u16>()
        .map_err(|_| "invalid port")?;
    let enabled = map
        .get("enabled")
        .ok_or("missing enabled")?
        .parse::<bool>()
        .map_err(|_| "invalid enabled")?;

    Ok(Entry { name, port, enabled })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_entry_with_blank_and_comment_lines() {
        let input = "\n# service config\nname = api\n\nport = 8080\nenabled = true\n";
        let got = parse_entry(input).unwrap();
        assert_eq!(
            got,
            Entry {
                name: "api".to_string(),
                port: 8080,
                enabled: true,
            }
        );
    }

    #[test]
    fn rejects_port_zero() {
        let err = parse_entry("name=db\nport=0\nenabled=false\n").unwrap_err();
        assert_eq!(err, "port out of range");
    }

    #[test]
    fn rejects_empty_name() {
        let err = parse_entry("name=   \nport=12\nenabled=true\n").unwrap_err();
        assert_eq!(err, "empty name");
    }

    #[test]
    fn rejects_unknown_field() {
        let err = parse_entry("name=api\nport=90\nenabled=true\nmode=dev\n").unwrap_err();
        assert_eq!(err, "unknown field: mode");
    }
}
