#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    Member,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    View,
    Comment,
    Delete,
    Suspend,
}

pub fn is_allowed(role: Role, action: Action, owner: bool, flagged: bool) -> bool {
    match action {
        Action::View => true,
        Action::Comment => match role {
            Role::Guest => false,
            Role::Member | Role::Admin => true,
        },
        Action::Delete => match role {
            Role::Admin => true,
            Role::Member => owner,
            Role::Guest => false,
        },
        Action::Suspend => match role {
            Role::Admin => owner,
            Role::Member => flagged,
            Role::Guest => false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{is_allowed, Action::*, Role::*};

    #[test]
    fn viewing_is_open_to_all_roles() {
        assert!(is_allowed(Guest, View, false, false));
        assert!(is_allowed(Member, View, true, true));
        assert!(is_allowed(Admin, View, false, true));
    }

    #[test]
    fn comment_requires_signed_in_user() {
        assert!(!is_allowed(Guest, Comment, false, false));
        assert!(is_allowed(Member, Comment, false, false));
        assert!(is_allowed(Admin, Comment, true, true));
    }

    #[test]
    fn delete_allows_admin_or_owner_member_only() {
        assert!(is_allowed(Admin, Delete, false, false));
        assert!(is_allowed(Member, Delete, true, false));
        assert!(!is_allowed(Member, Delete, false, false));
        assert!(!is_allowed(Guest, Delete, true, false));
    }

    #[test]
    fn suspend_requires_admin_and_never_owner() {
        assert!(is_allowed(Admin, Suspend, false, false));
        assert!(is_allowed(Admin, Suspend, false, true));
        assert!(!is_allowed(Admin, Suspend, true, false));
        assert!(!is_allowed(Member, Suspend, false, true));
        assert!(!is_allowed(Guest, Suspend, false, true));
    }
}
