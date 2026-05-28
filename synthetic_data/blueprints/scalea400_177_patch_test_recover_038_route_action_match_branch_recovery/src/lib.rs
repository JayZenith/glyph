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
    Reports,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    List,
    Create,
    Remove,
    Audit,
    Unsupported,
}

pub fn action_for(method: Method, resource: Resource, authenticated: bool) -> Action {
    match resource {
        Resource::Users => match method {
            Method::Get => Action::List,
            Method::Post => Action::Create,
            Method::Delete => Action::Remove,
        },
        Resource::Sessions => match method {
            Method::Get => Action::Unsupported,
            Method::Post => Action::Create,
            Method::Delete => Action::Unsupported,
        },
        Resource::Reports => match method {
            Method::Get => {
                if authenticated {
                    Action::List
                } else {
                    Action::Unsupported
                }
            }
            Method::Post => Action::Create,
            Method::Delete => Action::Remove,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn users_routes_follow_crud_shape() {
        assert_eq!(action_for(Method::Get, Resource::Users, false), Action::List);
        assert_eq!(action_for(Method::Post, Resource::Users, true), Action::Create);
        assert_eq!(action_for(Method::Delete, Resource::Users, true), Action::Remove);
    }

    #[test]
    fn sessions_only_allow_login_and_logout() {
        assert_eq!(action_for(Method::Get, Resource::Sessions, true), Action::Unsupported);
        assert_eq!(action_for(Method::Post, Resource::Sessions, false), Action::Create);
        assert_eq!(action_for(Method::Delete, Resource::Sessions, true), Action::Remove);
    }

    #[test]
    fn reports_require_auth_and_never_create() {
        assert_eq!(action_for(Method::Get, Resource::Reports, true), Action::Audit);
        assert_eq!(action_for(Method::Get, Resource::Reports, false), Action::Unsupported);
        assert_eq!(action_for(Method::Post, Resource::Reports, true), Action::Unsupported);
        assert_eq!(action_for(Method::Delete, Resource::Reports, true), Action::Remove);
    }
}
