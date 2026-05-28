#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Create { dry_run: bool },
    Delete { force: bool },
    Sync { full: bool },
    Inspect,
}

pub fn dispatch(action: Action) -> &'static str {
    match action {
        Action::Create { .. } => "create",
        Action::Delete { .. } => "delete",
        Action::Sync { .. } => "sync",
        Action::Inspect => "inspect",
    }
}

pub fn execute(action: Action) -> &'static str {
    match action {
        Action::Create { dry_run } => {
            if dry_run {
                "create"
            } else {
                "created"
            }
        }
        Action::Delete { force } => {
            if force {
                "deleted"
            } else {
                "delete"
            }
        }
        Action::Sync { full } => {
            if full {
                "synced"
            } else {
                "sync"
            }
        }
        Action::Inspect => "inspect",
    }
}

#[cfg(test)]
mod tests {
    use super::{dispatch, execute, Action};

    #[test]
    fn dispatch_routes_all_variants() {
        assert_eq!(dispatch(Action::Create { dry_run: false }), "create");
        assert_eq!(dispatch(Action::Delete { force: false }), "delete");
        assert_eq!(dispatch(Action::Sync { full: false }), "sync");
        assert_eq!(dispatch(Action::Inspect), "inspect");
    }

    #[test]
    fn execute_handles_flagged_variants() {
        assert_eq!(execute(Action::Create { dry_run: true }), "planned create");
        assert_eq!(execute(Action::Create { dry_run: false }), "created");
        assert_eq!(execute(Action::Delete { force: true }), "force deleted");
        assert_eq!(execute(Action::Delete { force: false }), "delete blocked");
        assert_eq!(execute(Action::Sync { full: true }), "full sync");
        assert_eq!(execute(Action::Sync { full: false }), "sync incremental");
        assert_eq!(execute(Action::Inspect), "inspect");
    }
}
