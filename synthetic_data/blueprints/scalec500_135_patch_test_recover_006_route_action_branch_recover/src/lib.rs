#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    Member,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    View,
    Edit,
    Delete,
    Transfer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Article { archived: bool, owned: bool },
    Comment { flagged: bool, owned: bool },
    Account { same_user: bool },
}

pub fn decide(role: Role, action: Action, resource: Resource) -> &'static str {
    match (role, action, resource) {
        (_, Action::View, Resource::Article { .. }) => "allow",
        (Role::Guest, Action::Edit, _) => "deny",
        (Role::Member, Action::Edit, Resource::Article { archived, owned }) => {
            if archived || owned {
                "allow"
            } else {
                "review"
            }
        }
        (Role::Admin, Action::Delete, _) => "allow",
        (Role::Member, Action::Delete, Resource::Comment { flagged, .. }) => {
            if flagged {
                "review"
            } else {
                "deny"
            }
        }
        (Role::Member, Action::Transfer, Resource::Account { same_user }) => {
            if same_user {
                "allow"
            } else {
                "review"
            }
        }
        (Role::Admin, Action::Transfer, Resource::Account { .. }) => "review",
        _ => "deny",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article_view_is_public() {
        assert_eq!(decide(Role::Guest, Action::View, Resource::Article { archived: true, owned: false }), "allow");
    }

    #[test]
    fn member_can_edit_only_owned_active_article() {
        assert_eq!(decide(Role::Member, Action::Edit, Resource::Article { archived: false, owned: true }), "allow");
        assert_eq!(decide(Role::Member, Action::Edit, Resource::Article { archived: false, owned: false }), "review");
        assert_eq!(decide(Role::Member, Action::Edit, Resource::Article { archived: true, owned: true }), "deny");
    }

    #[test]
    fn member_delete_flagged_comment_is_denied() {
        assert_eq!(decide(Role::Member, Action::Delete, Resource::Comment { flagged: true, owned: false }), "deny");
        assert_eq!(decide(Role::Member, Action::Delete, Resource::Comment { flagged: false, owned: true }), "deny");
    }

    #[test]
    fn transfer_requires_admin_and_same_user_account() {
        assert_eq!(decide(Role::Member, Action::Transfer, Resource::Account { same_user: true }), "deny");
        assert_eq!(decide(Role::Admin, Action::Transfer, Resource::Account { same_user: true }), "allow");
        assert_eq!(decide(Role::Admin, Action::Transfer, Resource::Account { same_user: false }), "review");
    }
}
