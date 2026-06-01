#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub debug: Option<bool>,
    pub tags: Option<Vec<String>>,
}

pub fn merge_config(defaults: &Config, file: Option<&PartialConfig>, cli: Option<&PartialConfig>) -> Config {
    let mut merged = defaults.clone();

    if let Some(file) = file {
        if let Some(host) = &file.host {
            merged.host = host.clone();
        }
        if let Some(port) = file.port {
            merged.port = port;
        }
        if let Some(debug) = file.debug {
            merged.debug = debug;
        }
        if let Some(tags) = &file.tags {
            if !tags.is_empty() {
                merged.tags = tags.clone();
            }
        }
    }

    if let Some(cli) = cli {
        if let Some(host) = &cli.host {
            merged.host = host.clone();
        }
        if let Some(port) = cli.port {
            merged.port = port;
        }
        if let Some(debug) = cli.debug {
            merged.debug = debug;
        }
        if let Some(tags) = &cli.tags {
            merged.tags.extend(tags.clone());
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Config {
        Config {
            host: "localhost".into(),
            port: 8080,
            debug: false,
            tags: vec!["base".into()],
        }
    }

    #[test]
    fn file_values_override_defaults() {
        let file = PartialConfig {
            host: Some("filehost".into()),
            port: Some(9000),
            debug: Some(true),
            tags: Some(vec!["file".into()]),
        };

        let merged = merge_config(&defaults(), Some(&file), None);
        assert_eq!(merged.host, "filehost");
        assert_eq!(merged.port, 9000);
        assert!(merged.debug);
        assert_eq!(merged.tags, vec!["file"]);
    }

    #[test]
    fn cli_values_override_file_and_defaults() {
        let file = PartialConfig {
            host: Some("filehost".into()),
            port: Some(9000),
            debug: Some(false),
            tags: Some(vec!["file".into()]),
        };
        let cli = PartialConfig {
            host: Some("clihost".into()),
            port: Some(7000),
            debug: Some(true),
            tags: Some(vec!["cli".into()]),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&cli));
        assert_eq!(merged.host, "clihost");
        assert_eq!(merged.port, 7000);
        assert!(merged.debug);
        assert_eq!(merged.tags, vec!["cli"]);
    }

    #[test]
    fn explicit_empty_tags_clear_previous_layers() {
        let file = PartialConfig {
            host: None,
            port: None,
            debug: None,
            tags: Some(vec![]),
        };

        let merged = merge_config(&defaults(), Some(&file), None);
        assert!(merged.tags.is_empty());
    }

    #[test]
    fn later_empty_cli_tags_can_clear_file_tags() {
        let file = PartialConfig {
            host: None,
            port: None,
            debug: None,
            tags: Some(vec!["file".into()]),
        };
        let cli = PartialConfig {
            host: None,
            port: None,
            debug: None,
            tags: Some(vec![]),
        };

        let merged = merge_config(&defaults(), Some(&file), Some(&cli));
        assert!(merged.tags.is_empty());
    }
}
