#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Create,
    Update { fields_changed: usize },
    Delete { hard: bool },
    Access { authenticated: bool, write: bool },
    Export { format: ExportFormat, rows: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportFormat {
    Csv,
    Json,
    Xml,
}

pub fn route(action: Action) -> &'static str {
    match action {
        Action::Create => "audit",
        Action::Update { fields_changed } => {
            if fields_changed == 0 {
                "ignore"
            } else {
                "audit"
            }
        }
        Action::Delete { hard } => {
            if hard {
                "audit"
            } else {
                "ignore"
            }
        }
        Action::Access {
            authenticated,
            write,
        } => {
            if authenticated && write {
                "audit"
            } else {
                "ignore"
            }
        }
        Action::Export { format, rows } => match format {
            ExportFormat::Csv => "ignore",
            ExportFormat::Json => {
                if rows > 1000 {
                    "audit"
                } else {
                    "ignore"
                }
            }
            ExportFormat::Xml => {
                if rows > 0 {
                    "audit"
                } else {
                    "ignore"
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_substantive_update_are_audited() {
        assert_eq!(route(Action::Create), "audit");
        assert_eq!(
            route(Action::Update { fields_changed: 2 }),
            "audit"
        );
        assert_eq!(
            route(Action::Update { fields_changed: 0 }),
            "ignore"
        );
    }

    #[test]
    fn both_delete_kinds_are_audited() {
        assert_eq!(route(Action::Delete { hard: true }), "audit");
        assert_eq!(route(Action::Delete { hard: false }), "audit");
    }

    #[test]
    fn access_rules_distinguish_auth_from_write_attempts() {
        assert_eq!(
            route(Action::Access {
                authenticated: true,
                write: true,
            }),
            "audit"
        );
        assert_eq!(
            route(Action::Access {
                authenticated: false,
                write: true,
            }),
            "security"
        );
        assert_eq!(
            route(Action::Access {
                authenticated: true,
                write: false,
            }),
            "ignore"
        );
    }

    #[test]
    fn export_rules_depend_on_format_and_row_count() {
        assert_eq!(
            route(Action::Export {
                format: ExportFormat::Csv,
                rows: 0,
            }),
            "ignore"
        );
        assert_eq!(
            route(Action::Export {
                format: ExportFormat::Csv,
                rows: 10,
            }),
            "audit"
        );
        assert_eq!(
            route(Action::Export {
                format: ExportFormat::Json,
                rows: 1000,
            }),
            "audit"
        );
        assert_eq!(
            route(Action::Export {
                format: ExportFormat::Json,
                rows: 50,
            }),
            "ignore"
        );
        assert_eq!(
            route(Action::Export {
                format: ExportFormat::Xml,
                rows: 1,
            }),
            "security"
        );
    }
}
