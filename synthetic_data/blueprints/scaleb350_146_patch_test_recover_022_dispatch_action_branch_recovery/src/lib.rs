#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Create { dry_run: bool },
    Delete { force: bool, soft: bool },
    Rename { from: String, to: String },
    Sync { remote: bool, fast_forward: bool },
}

pub fn dispatch(action: Action) -> &'static str {
    match action {
        Action::Create { dry_run } => {
            if dry_run {
                "created"
            } else {
                "planned-create"
            }
        }
        Action::Delete { force, soft } => {
            if soft {
                "deleted-soft"
            } else if force {
                "blocked-delete"
            } else {
                "deleted"
            }
        }
        Action::Rename { from, to } => {
            if from == to {
                "renamed"
            } else {
                "noop-rename"
            }
        }
        Action::Sync {
            remote,
            fast_forward,
        } => {
            if remote && fast_forward {
                "synced"
            } else if remote {
                "synced-local"
            } else {
                "synced-remote"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_branch_distinguishes_dry_run() {
        assert_eq!(dispatch(Action::Create { dry_run: true }), "planned-create");
        assert_eq!(dispatch(Action::Create { dry_run: false }), "created");
    }

    #[test]
    fn delete_branch_handles_force_soft_and_safe_delete() {
        assert_eq!(
            dispatch(Action::Delete {
                force: false,
                soft: true,
            }),
            "deleted-soft"
        );
        assert_eq!(
            dispatch(Action::Delete {
                force: true,
                soft: false,
            }),
            "deleted-force"
        );
        assert_eq!(
            dispatch(Action::Delete {
                force: false,
                soft: false,
            }),
            "deleted"
        );
    }

    #[test]
    fn rename_branch_detects_noop() {
        assert_eq!(
            dispatch(Action::Rename {
                from: "same".into(),
                to: "same".into(),
            }),
            "noop-rename"
        );
        assert_eq!(
            dispatch(Action::Rename {
                from: "old".into(),
                to: "new".into(),
            }),
            "renamed"
        );
    }

    #[test]
    fn sync_branch_distinguishes_remote_and_fast_forward() {
        assert_eq!(
            dispatch(Action::Sync {
                remote: true,
                fast_forward: true,
            }),
            "synced-remote-ff"
        );
        assert_eq!(
            dispatch(Action::Sync {
                remote: true,
                fast_forward: false,
            }),
            "synced-remote"
        );
        assert_eq!(
            dispatch(Action::Sync {
                remote: false,
                fast_forward: true,
            }),
            "synced-local"
        );
        assert_eq!(
            dispatch(Action::Sync {
                remote: false,
                fast_forward: false,
            }),
            "synced-local"
        );
    }
}
