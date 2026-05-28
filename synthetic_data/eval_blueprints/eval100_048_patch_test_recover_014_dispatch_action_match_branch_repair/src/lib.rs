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
    Ban,
}

pub fn dispatch(role: Role, action: Action, suspended: bool) -> &'static str {
    if suspended {
        return match action {
            Action::View => "read-only",
            _ => "denied",
        };
    }

    match (role, action) {
        (Role::Guest, Action::View) => "allowed",
        (Role::Guest, Action::Comment) => "allowed",
        (Role::Guest, Action::Delete) => "denied",
        (Role::Guest, Action::Ban) => "denied",

        (Role::Member, Action::View) => "allowed",
        (Role::Member, Action::Comment) => "allowed",
        (Role::Member, Action::Delete) => "allowed",
        (Role::Member, Action::Ban) => "denied",

        (Role::Admin, Action::View) => "allowed",
        (Role::Admin, Action::Comment) => "allowed",
        (Role::Admin, Action::Delete) => "allowed",
        (Role::Admin, Action::Ban) => "denied",
    }
}

#[cfg(test)]
mod tests {
    use super::{dispatch, Action::*, Role::*};

    #[test]
    fn guests_can_only_view() {
        assert_eq!(dispatch(Guest, View, false), "allowed");
        assert_eq!(dispatch(Guest, Comment, false), "denied");
        assert_eq!(dispatch(Guest, Delete, false), "denied");
        assert_eq!(dispatch(Guest, Ban, false), "denied");
    }

    #[test]
    fn members_cannot_delete_or_ban() {
        assert_eq!(dispatch(Member, View, false), "allowed");
        assert_eq!(dispatch(Member, Comment, false), "allowed");
        assert_eq!(dispatch(Member, Delete, false), "denied");
        assert_eq!(dispatch(Member, Ban, false), "denied");
    }

    #[test]
    fn admins_can_do_everything_when_active() {
        assert_eq!(dispatch(Admin, View, false), "allowed");
        assert_eq!(dispatch(Admin, Comment, false), "allowed");
        assert_eq!(dispatch(Admin, Delete, false), "allowed");
        assert_eq!(dispatch(Admin, Ban, false), "allowed");
    }

    #[test]
    fn suspended_users_are_read_only_even_admins() {
        assert_eq!(dispatch(Admin, View, true), "read-only");
        assert_eq!(dispatch(Admin, Comment, true), "denied");
        assert_eq!(dispatch(Member, Delete, true), "denied");
    }
}
