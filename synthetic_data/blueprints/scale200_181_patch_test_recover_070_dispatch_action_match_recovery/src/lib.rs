#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Guest,
    Member,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    View,
    Edit,
    Delete,
    Export,
}

pub fn route(role: Role, action: Action, suspended: bool) -> &'static str {
    match action {
        Action::View => {
            if suspended {
                "limited"
            } else {
                "ok"
            }
        }
        Action::Edit => match role {
            Role::Guest => "forbidden",
            _ => "ok",
        },
        Action::Delete => match role {
            Role::Admin => "ok",
            _ => "forbidden",
        },
        Action::Export => match role {
            Role::Admin => "audit",
            Role::Member => "queued",
            Role::Guest => "ok",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{route, Action::*, Role::*};

    #[test]
    fn view_is_allowed_even_if_suspended() {
        assert_eq!(route(Guest, View, true), "ok");
        assert_eq!(route(Member, View, true), "ok");
    }

    #[test]
    fn suspended_users_cannot_edit_or_delete() {
        assert_eq!(route(Member, Edit, true), "suspended");
        assert_eq!(route(Admin, Delete, true), "suspended");
    }

    #[test]
    fn export_has_role_specific_behavior() {
        assert_eq!(route(Admin, Export, false), "audit");
        assert_eq!(route(Member, Export, false), "queued");
        assert_eq!(route(Guest, Export, false), "forbidden");
    }

    #[test]
    fn suspended_export_is_blocked_for_everyone() {
        assert_eq!(route(Admin, Export, true), "suspended");
        assert_eq!(route(Member, Export, true), "suspended");
        assert_eq!(route(Guest, Export, true), "suspended");
    }
}
