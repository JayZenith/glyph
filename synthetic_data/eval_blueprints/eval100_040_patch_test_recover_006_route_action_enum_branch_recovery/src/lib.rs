#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    User,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Allow,
    RedirectLogin,
    Reject(&'static str),
}

pub fn decide_action(method: Method, path: &str, role: Role) -> Action {
    match (method, path, role) {
        (_, "/health", _) => Action::Allow,
        (Method::Get, "/admin", Role::Admin) => Action::Allow,
        (Method::Get, "/admin", _) => Action::RedirectLogin,
        (Method::Delete, "/admin", Role::Admin) => Action::Allow,
        (Method::Delete, "/admin", _) => Action::RedirectLogin,
        (Method::Post, "/session", Role::Guest) => Action::Allow,
        (Method::Post, "/session", _) => Action::Reject("session_exists"),
        (Method::Post, "/posts", Role::User | Role::Admin) => Action::Allow,
        (Method::Delete, "/posts", Role::Admin) => Action::Allow,
        (Method::Delete, "/posts", _) => Action::RedirectLogin,
        _ => Action::Reject("not_found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_is_open_for_everyone() {
        assert_eq!(decide_action(Method::Get, "/health", Role::Guest), Action::Allow);
        assert_eq!(decide_action(Method::Delete, "/health", Role::Admin), Action::Allow);
    }

    #[test]
    fn admin_get_requires_admin_and_non_admins_are_forbidden() {
        assert_eq!(decide_action(Method::Get, "/admin", Role::Admin), Action::Allow);
        assert_eq!(
            decide_action(Method::Get, "/admin", Role::User),
            Action::Reject("forbidden")
        );
        assert_eq!(
            decide_action(Method::Get, "/admin", Role::Guest),
            Action::RedirectLogin
        );
    }

    #[test]
    fn admin_delete_requires_admin_and_non_admins_are_forbidden() {
        assert_eq!(decide_action(Method::Delete, "/admin", Role::Admin), Action::Allow);
        assert_eq!(
            decide_action(Method::Delete, "/admin", Role::User),
            Action::Reject("forbidden")
        );
        assert_eq!(
            decide_action(Method::Delete, "/admin", Role::Guest),
            Action::RedirectLogin
        );
    }

    #[test]
    fn session_creation_only_for_guests() {
        assert_eq!(decide_action(Method::Post, "/session", Role::Guest), Action::Allow);
        assert_eq!(
            decide_action(Method::Post, "/session", Role::User),
            Action::Reject("session_exists")
        );
    }

    #[test]
    fn post_creation_requires_signed_in_user() {
        assert_eq!(decide_action(Method::Post, "/posts", Role::User), Action::Allow);
        assert_eq!(decide_action(Method::Post, "/posts", Role::Admin), Action::Allow);
        assert_eq!(
            decide_action(Method::Post, "/posts", Role::Guest),
            Action::RedirectLogin
        );
    }

    #[test]
    fn post_delete_requires_admin_but_logged_in_users_are_forbidden() {
        assert_eq!(decide_action(Method::Delete, "/posts", Role::Admin), Action::Allow);
        assert_eq!(
            decide_action(Method::Delete, "/posts", Role::User),
            Action::Reject("forbidden")
        );
        assert_eq!(
            decide_action(Method::Delete, "/posts", Role::Guest),
            Action::RedirectLogin
        );
    }

    #[test]
    fn unknown_routes_still_report_not_found() {
        assert_eq!(
            decide_action(Method::Get, "/missing", Role::Admin),
            Action::Reject("not_found")
        );
    }
}
