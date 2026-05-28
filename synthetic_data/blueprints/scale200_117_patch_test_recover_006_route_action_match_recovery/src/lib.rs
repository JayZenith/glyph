#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    ListUsers,
    GetUser,
    CreateUser,
    DeleteUser,
    Health,
    MethodNotAllowed,
    NotFound,
}

pub fn plan_action(method: Method, path: &str) -> Action {
    match (method, path) {
        (Method::Get, "/users") => Action::ListUsers,
        (Method::Get, p) if p.starts_with("/users/") => Action::GetUser,
        (Method::Post, p) if p.starts_with("/users/") => Action::CreateUser,
        (Method::Delete, "/users") => Action::DeleteUser,
        (_, "/health") => Action::MethodNotAllowed,
        _ => Action::NotFound,
    }
}

#[cfg(test)]
mod tests {
    use super::{plan_action, Action, Method};

    #[test]
    fn get_routes_match_expected_actions() {
        assert_eq!(plan_action(Method::Get, "/users"), Action::ListUsers);
        assert_eq!(plan_action(Method::Get, "/users/42"), Action::GetUser);
        assert_eq!(plan_action(Method::Get, "/health"), Action::Health);
    }

    #[test]
    fn post_and_delete_routes_are_distinct() {
        assert_eq!(plan_action(Method::Post, "/users"), Action::CreateUser);
        assert_eq!(plan_action(Method::Delete, "/users/42"), Action::DeleteUser);
    }

    #[test]
    fn unsupported_methods_on_known_paths_report_method_not_allowed() {
        assert_eq!(plan_action(Method::Delete, "/health"), Action::MethodNotAllowed);
        assert_eq!(plan_action(Method::Post, "/users/42"), Action::MethodNotAllowed);
    }

    #[test]
    fn unknown_paths_are_not_found() {
        assert_eq!(plan_action(Method::Get, "/metrics"), Action::NotFound);
        assert_eq!(plan_action(Method::Delete, "/admin"), Action::NotFound);
    }
}
