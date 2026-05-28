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
    Publish,
    Archive,
}

pub fn decision(role: Role, action: Action, suspended: bool, owns_item: bool) -> &'static str {
    match action {
        Action::View => "allow",
        Action::Edit => match role {
            Role::Guest => "deny",
            Role::Member => {
                if suspended {
                    "deny"
                } else if owns_item {
                    "allow"
                } else {
                    "review"
                }
            }
            Role::Admin => {
                if suspended {
                    "review"
                } else {
                    "allow"
                }
            }
        },
        Action::Delete => match role {
            Role::Admin => {
                if suspended {
                    "review"
                } else {
                    "allow"
                }
            }
            Role::Member => {
                if suspended {
                    "deny"
                } else if owns_item {
                    "review"
                } else {
                    "deny"
                }
            }
            Role::Guest => "deny",
        },
        Action::Publish => match role {
            Role::Admin => "allow",
            Role::Member => {
                if suspended {
                    "deny"
                } else {
                    "review"
                }
            }
            Role::Guest => "deny",
        },
        Action::Archive => match role {
            Role::Admin => "allow",
            Role::Member => {
                if owns_item {
                    "allow"
                } else {
                    "review"
                }
            }
            Role::Guest => "deny",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{decision, Action::*, Role::*};

    #[test]
    fn guests_can_only_view() {
        assert_eq!(decision(Guest, View, false, false), "allow");
        assert_eq!(decision(Guest, Edit, false, true), "deny");
        assert_eq!(decision(Guest, Delete, false, true), "deny");
        assert_eq!(decision(Guest, Publish, false, false), "deny");
        assert_eq!(decision(Guest, Archive, false, true), "deny");
    }

    #[test]
    fn member_edit_and_delete_depend_on_state() {
        assert_eq!(decision(Member, Edit, false, true), "allow");
        assert_eq!(decision(Member, Edit, false, false), "review");
        assert_eq!(decision(Member, Edit, true, true), "deny");

        assert_eq!(decision(Member, Delete, false, true), "review");
        assert_eq!(decision(Member, Delete, false, false), "deny");
        assert_eq!(decision(Member, Delete, true, true), "deny");
    }

    #[test]
    fn publish_requires_extra_review_for_admin_if_suspended() {
        assert_eq!(decision(Admin, Publish, false, false), "allow");
        assert_eq!(decision(Admin, Publish, true, false), "review");
    }

    #[test]
    fn archive_is_never_allowed_for_suspended_member() {
        assert_eq!(decision(Member, Archive, true, true), "deny");
        assert_eq!(decision(Member, Archive, true, false), "deny");
        assert_eq!(decision(Member, Archive, false, true), "allow");
        assert_eq!(decision(Member, Archive, false, false), "review");
    }
}
