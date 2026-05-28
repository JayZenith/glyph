#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    User,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    View,
    Edit,
    Delete,
    Publish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Page { locked: bool },
    Comment { flagged: bool },
    System,
}

pub fn decide(role: Role, action: Action, resource: Resource) -> &'static str {
    match (role, action, resource) {
        (Role::Guest, Action::View, Resource::Page { .. }) => "allow",
        (Role::Guest, Action::View, Resource::Comment { .. }) => "allow",
        (Role::Guest, _, _) => "deny",

        (Role::User, Action::View, _) => "allow",
        (Role::User, Action::Edit, Resource::Page { locked }) => {
            if locked { "review" } else { "allow" }
        }
        (Role::User, Action::Edit, Resource::Comment { flagged }) => {
            if flagged { "deny" } else { "allow" }
        }
        (Role::User, Action::Delete, Resource::Comment { .. }) => "allow",
        (Role::User, Action::Publish, Resource::Page { .. }) => "allow",
        (Role::User, _, _) => "deny",

        (Role::Admin, Action::Delete, Resource::System) => "deny",
        (Role::Admin, _, Resource::System) => "allow",
        (Role::Admin, _, Resource::Page { locked: true }) => "review",
        (Role::Admin, _, _) => "allow",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guest_can_only_view_non_system_content() {
        assert_eq!(decide(Role::Guest, Action::View, Resource::Page { locked: false }), "allow");
        assert_eq!(decide(Role::Guest, Action::View, Resource::Comment { flagged: true }), "allow");
        assert_eq!(decide(Role::Guest, Action::View, Resource::System), "deny");
        assert_eq!(decide(Role::Guest, Action::Edit, Resource::Page { locked: false }), "deny");
    }

    #[test]
    fn user_rules_distinguish_locked_and_flagged_items() {
        assert_eq!(decide(Role::User, Action::Edit, Resource::Page { locked: false }), "allow");
        assert_eq!(decide(Role::User, Action::Edit, Resource::Page { locked: true }), "review");
        assert_eq!(decide(Role::User, Action::Edit, Resource::Comment { flagged: false }), "allow");
        assert_eq!(decide(Role::User, Action::Edit, Resource::Comment { flagged: true }), "deny");
    }

    #[test]
    fn users_have_limited_delete_and_publish_rights() {
        assert_eq!(decide(Role::User, Action::Delete, Resource::Comment { flagged: false }), "review");
        assert_eq!(decide(Role::User, Action::Delete, Resource::Page { locked: false }), "deny");
        assert_eq!(decide(Role::User, Action::Publish, Resource::Page { locked: false }), "review");
        assert_eq!(decide(Role::User, Action::Publish, Resource::Comment { flagged: false }), "deny");
    }

    #[test]
    fn admin_rules_keep_system_delete_blocked_but_bypass_reviews_elsewhere() {
        assert_eq!(decide(Role::Admin, Action::Delete, Resource::System), "deny");
        assert_eq!(decide(Role::Admin, Action::Publish, Resource::System), "allow");
        assert_eq!(decide(Role::Admin, Action::Edit, Resource::Page { locked: true }), "allow");
        assert_eq!(decide(Role::Admin, Action::Delete, Resource::Comment { flagged: true }), "allow");
    }
}
