#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Create,
    Update,
    Delete,
    Archive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    Member,
    Moderator,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Own,
    Team,
    Global,
}

pub fn route(action: Action, role: Role, scope: Scope) -> &'static str {
    match action {
        Action::Create => match role {
            Role::Guest => "reject",
            Role::Member => "queue",
            Role::Moderator | Role::Admin => "write",
        },
        Action::Update => match (role, scope) {
            (Role::Guest, _) => "reject",
            (Role::Member, Scope::Own) => "write",
            (Role::Member, _) => "queue",
            (Role::Moderator, Scope::Global) => "queue",
            (Role::Moderator, _) => "write",
            (Role::Admin, _) => "write",
        },
        Action::Delete => match role {
            Role::Admin => "write",
            Role::Moderator => "queue",
            _ => "reject",
        },
        Action::Archive => match (role, scope) {
            (Role::Admin, _) => "write",
            (Role::Moderator, Scope::Team) => "write",
            (Role::Moderator, _) => "queue",
            (Role::Member, Scope::Own) => "write",
            (Role::Member, _) => "queue",
            _ => "reject",
        },
    }
}

pub fn should_audit(action: Action, role: Role, scope: Scope) -> bool {
    match action {
        Action::Create => matches!(role, Role::Guest),
        Action::Update => matches!((role, scope), (Role::Moderator, Scope::Global)),
        Action::Delete => !matches!(role, Role::Guest),
        Action::Archive => matches!(scope, Scope::Global),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_routes_by_role() {
        assert_eq!(route(Action::Create, Role::Guest, Scope::Own), "reject");
        assert_eq!(route(Action::Create, Role::Member, Scope::Team), "queue");
        assert_eq!(route(Action::Create, Role::Moderator, Scope::Global), "write");
    }

    #[test]
    fn update_respects_scope() {
        assert_eq!(route(Action::Update, Role::Member, Scope::Own), "write");
        assert_eq!(route(Action::Update, Role::Member, Scope::Team), "queue");
        assert_eq!(route(Action::Update, Role::Moderator, Scope::Team), "write");
        assert_eq!(route(Action::Update, Role::Moderator, Scope::Global), "queue");
    }

    #[test]
    fn delete_requires_stronger_privilege() {
        assert_eq!(route(Action::Delete, Role::Member, Scope::Own), "reject");
        assert_eq!(route(Action::Delete, Role::Moderator, Scope::Team), "queue");
        assert_eq!(route(Action::Delete, Role::Admin, Scope::Global), "write");
    }

    #[test]
    fn archive_rules_distinguish_team_and_global() {
        assert_eq!(route(Action::Archive, Role::Guest, Scope::Own), "reject");
        assert_eq!(route(Action::Archive, Role::Member, Scope::Own), "queue");
        assert_eq!(route(Action::Archive, Role::Moderator, Scope::Team), "write");
        assert_eq!(route(Action::Archive, Role::Moderator, Scope::Global), "reject");
        assert_eq!(route(Action::Archive, Role::Admin, Scope::Global), "write");
    }

    #[test]
    fn auditing_only_flags_sensitive_allowed_cases() {
        assert!(!should_audit(Action::Create, Role::Member, Scope::Own));
        assert!(should_audit(Action::Delete, Role::Moderator, Scope::Team));
        assert!(!should_audit(Action::Delete, Role::Member, Scope::Own));
        assert!(should_audit(Action::Archive, Role::Admin, Scope::Global));
        assert!(!should_audit(Action::Archive, Role::Moderator, Scope::Team));
    }
}
