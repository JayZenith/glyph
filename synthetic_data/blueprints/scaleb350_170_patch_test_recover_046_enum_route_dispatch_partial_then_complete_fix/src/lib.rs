#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Users,
    Sessions,
    Health,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Auth {
    Anonymous,
    User,
    Admin,
}

pub fn dispatch(method: Method, resource: Resource, auth: Auth) -> &'static str {
    match (method, resource, auth) {
        (Method::Get, Resource::Users, _) => "users:list",
        (Method::Get, Resource::Sessions, _) => "sessions:list",
        (Method::Get, Resource::Health, _) => "health:check",
        (Method::Post, Resource::Users, Auth::Anonymous) => "users:create",
        (Method::Post, Resource::Users, _) => "users:update",
        (Method::Post, Resource::Sessions, _) => "sessions:create",
        (Method::Post, Resource::Health, _) => "health:unsupported",
        (Method::Delete, Resource::Users, _) => "users:delete",
        (Method::Delete, Resource::Sessions, _) => "sessions:delete",
        (Method::Delete, Resource::Health, _) => "health:unsupported",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_routes_ignore_auth() {
        assert_eq!(dispatch(Method::Get, Resource::Users, Auth::Anonymous), "users:list");
        assert_eq!(dispatch(Method::Get, Resource::Sessions, Auth::Admin), "sessions:list");
        assert_eq!(dispatch(Method::Get, Resource::Health, Auth::User), "health:check");
    }

    #[test]
    fn user_post_requires_non_anonymous_update_path() {
        assert_eq!(dispatch(Method::Post, Resource::Users, Auth::Anonymous), "users:create");
        assert_eq!(dispatch(Method::Post, Resource::Users, Auth::User), "users:update");
        assert_eq!(dispatch(Method::Post, Resource::Users, Auth::Admin), "users:update");
    }

    #[test]
    fn session_routes_have_auth_specific_behavior() {
        assert_eq!(dispatch(Method::Post, Resource::Sessions, Auth::Anonymous), "sessions:create");
        assert_eq!(dispatch(Method::Post, Resource::Sessions, Auth::User), "sessions:refresh");
        assert_eq!(dispatch(Method::Post, Resource::Sessions, Auth::Admin), "sessions:impersonate");
        assert_eq!(dispatch(Method::Delete, Resource::Sessions, Auth::Anonymous), "sessions:forbidden");
        assert_eq!(dispatch(Method::Delete, Resource::Sessions, Auth::User), "sessions:revoke_self");
        assert_eq!(dispatch(Method::Delete, Resource::Sessions, Auth::Admin), "sessions:revoke_any");
    }

    #[test]
    fn deleting_users_is_admin_only() {
        assert_eq!(dispatch(Method::Delete, Resource::Users, Auth::Anonymous), "users:forbidden");
        assert_eq!(dispatch(Method::Delete, Resource::Users, Auth::User), "users:forbidden");
        assert_eq!(dispatch(Method::Delete, Resource::Users, Auth::Admin), "users:delete");
    }
}
