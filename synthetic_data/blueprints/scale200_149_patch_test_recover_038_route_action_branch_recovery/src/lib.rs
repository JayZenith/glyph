#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Users,
    UserById,
    Health,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    ListUsers,
    GetUser,
    CreateUser,
    DeleteUser,
    HealthCheck,
    MethodNotAllowed,
    NotFound,
}

pub fn route(method: Method, path: &str) -> Action {
    let resource = match path {
        "/users" => Resource::Users,
        "/users/:id" => Resource::UserById,
        "/health" => Resource::Health,
        _ if path.starts_with("/users/") && path[7..].chars().all(|c| c.is_ascii_digit()) => Resource::Users,
        _ => return Action::NotFound,
    };

    match (method, resource) {
        (Method::Get, Resource::Users) => Action::ListUsers,
        (Method::Post, Resource::Users) => Action::CreateUser,
        (Method::Delete, Resource::Users) => Action::DeleteUser,
        (_, Resource::Health) => Action::HealthCheck,
        _ => Action::MethodNotAllowed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn users_collection_routes() {
        assert_eq!(route(Method::Get, "/users"), Action::ListUsers);
        assert_eq!(route(Method::Post, "/users"), Action::CreateUser);
        assert_eq!(route(Method::Delete, "/users"), Action::DeleteUser);
    }

    #[test]
    fn user_id_routes_are_distinct_from_collection() {
        assert_eq!(route(Method::Get, "/users/42"), Action::GetUser);
        assert_eq!(route(Method::Delete, "/users/42"), Action::DeleteUser);
        assert_eq!(route(Method::Post, "/users/42"), Action::MethodNotAllowed);
    }

    #[test]
    fn health_is_get_only() {
        assert_eq!(route(Method::Get, "/health"), Action::HealthCheck);
        assert_eq!(route(Method::Post, "/health"), Action::MethodNotAllowed);
    }

    #[test]
    fn invalid_or_unknown_paths_are_not_found() {
        assert_eq!(route(Method::Get, "/users/abc"), Action::NotFound);
        assert_eq!(route(Method::Get, "/missing"), Action::NotFound);
    }
}
