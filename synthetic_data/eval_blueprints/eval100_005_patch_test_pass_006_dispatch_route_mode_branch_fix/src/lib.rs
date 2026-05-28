#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Root,
    Health,
    User,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Delete,
}

pub fn action(route: Route, method: Method, authenticated: bool) -> &'static str {
    match route {
        Route::Root => match method {
            Method::Get => "index",
            _ => "method_not_allowed",
        },
        Route::Health => match method {
            Method::Get => "ok",
            _ => "method_not_allowed",
        },
        Route::User => match method {
            Method::Get => {
                if authenticated {
                    "profile"
                } else {
                    "forbidden"
                }
            }
            Method::Post => "create_user",
            Method::Delete => "method_not_allowed",
        },
        Route::Admin => match method {
            Method::Get => {
                if authenticated {
                    "admin_panel"
                } else {
                    "forbidden"
                }
            }
            Method::Post => {
                if authenticated {
                    "admin_update"
                } else {
                    "forbidden"
                }
            }
            Method::Delete => {
                if authenticated {
                    "forbidden"
                } else {
                    "admin_remove"
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{action, Method, Route};

    #[test]
    fn public_routes_only_allow_get() {
        assert_eq!(action(Route::Root, Method::Get, false), "index");
        assert_eq!(action(Route::Health, Method::Get, false), "ok");
        assert_eq!(action(Route::Root, Method::Post, false), "method_not_allowed");
    }

    #[test]
    fn user_route_depends_on_auth_and_method() {
        assert_eq!(action(Route::User, Method::Get, true), "profile");
        assert_eq!(action(Route::User, Method::Get, false), "forbidden");
        assert_eq!(action(Route::User, Method::Post, false), "create_user");
        assert_eq!(action(Route::User, Method::Delete, true), "method_not_allowed");
    }

    #[test]
    fn admin_delete_requires_authentication() {
        assert_eq!(action(Route::Admin, Method::Delete, true), "admin_remove");
        assert_eq!(action(Route::Admin, Method::Delete, false), "forbidden");
    }

    #[test]
    fn admin_get_and_post_still_require_authentication() {
        assert_eq!(action(Route::Admin, Method::Get, true), "admin_panel");
        assert_eq!(action(Route::Admin, Method::Post, true), "admin_update");
        assert_eq!(action(Route::Admin, Method::Post, false), "forbidden");
    }
}
