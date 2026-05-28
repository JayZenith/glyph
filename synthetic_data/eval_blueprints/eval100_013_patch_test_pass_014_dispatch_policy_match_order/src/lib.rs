#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    User,
    Admin,
}

pub fn should_log(action: Action, role: Role, dry_run: bool) -> bool {
    match action {
        Action::Delete => !dry_run,
        Action::Create | Action::Update => match role {
            Role::Admin => true,
            Role::User => !dry_run,
            Role::Guest => false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{should_log, Action::*, Role::*};

    #[test]
    fn delete_is_logged_for_real_runs_by_non_guests() {
        assert!(should_log(Delete, Admin, false));
        assert!(should_log(Delete, User, false));
        assert!(!should_log(Delete, Guest, false));
    }

    #[test]
    fn delete_is_never_logged_in_dry_run() {
        assert!(!should_log(Delete, Admin, true));
        assert!(!should_log(Delete, User, true));
        assert!(!should_log(Delete, Guest, true));
    }

    #[test]
    fn create_and_update_follow_role_rules() {
        assert!(should_log(Create, Admin, false));
        assert!(should_log(Update, Admin, true));
        assert!(should_log(Create, User, false));
        assert!(!should_log(Update, User, true));
        assert!(!should_log(Create, Guest, false));
    }
}
