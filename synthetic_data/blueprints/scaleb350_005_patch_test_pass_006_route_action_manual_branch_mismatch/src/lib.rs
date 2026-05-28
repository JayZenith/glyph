#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Root,
    Users,
    UserDetail(u32),
    Search,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Delete,
}

pub fn action_for(route: Route, method: Method) -> &'static str {
    match route {
        Route::Root => "render_home",
        Route::Users => match method {
            Method::Get => "list_users",
            Method::Post => "create_user",
            Method::Delete => "method_not_allowed",
        },
        Route::UserDetail(_) => match method {
            Method::Get => "show_user",
            Method::Post => "create_user",
            Method::Delete => "delete_user",
        },
        Route::Search => match method {
            Method::Get => "search",
            _ => "method_not_allowed",
        },
        Route::Unknown => "not_found",
    }
}

#[cfg(test)]
mod tests {
    use super::{action_for, Method, Route};

    #[test]
    fn collection_and_root_routes_dispatch_correctly() {
        assert_eq!(action_for(Route::Root, Method::Get), "render_home");
        assert_eq!(action_for(Route::Users, Method::Get), "list_users");
        assert_eq!(action_for(Route::Users, Method::Post), "create_user");
        assert_eq!(action_for(Route::Users, Method::Delete), "method_not_allowed");
    }

    #[test]
    fn detail_route_has_its_own_post_behavior() {
        assert_eq!(action_for(Route::UserDetail(7), Method::Get), "show_user");
        assert_eq!(action_for(Route::UserDetail(7), Method::Post), "update_user");
        assert_eq!(action_for(Route::UserDetail(7), Method::Delete), "delete_user");
    }

    #[test]
    fn search_and_unknown_routes_reject_or_404() {
        assert_eq!(action_for(Route::Search, Method::Get), "search");
        assert_eq!(action_for(Route::Search, Method::Post), "method_not_allowed");
        assert_eq!(action_for(Route::Unknown, Method::Delete), "not_found");
    }
}
