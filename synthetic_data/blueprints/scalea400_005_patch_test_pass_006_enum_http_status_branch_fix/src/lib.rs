#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Health,
    Item,
    Admin,
}

pub fn status_for(method: Method, route: Route, authenticated: bool) -> u16 {
    match route {
        Route::Health => match method {
            Method::Get => 200,
            _ => 405,
        },
        Route::Item => match method {
            Method::Get => 200,
            Method::Post => 201,
            Method::Delete => 204,
        },
        Route::Admin => match method {
            Method::Get if authenticated => 200,
            Method::Post if authenticated => 204,
            _ => 405,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{status_for, Method, Route};

    #[test]
    fn health_only_allows_get() {
        assert_eq!(status_for(Method::Get, Route::Health, false), 200);
        assert_eq!(status_for(Method::Post, Route::Health, false), 405);
    }

    #[test]
    fn item_routes_have_method_specific_codes() {
        assert_eq!(status_for(Method::Get, Route::Item, false), 200);
        assert_eq!(status_for(Method::Post, Route::Item, false), 201);
        assert_eq!(status_for(Method::Delete, Route::Item, false), 204);
    }

    #[test]
    fn admin_requires_auth_and_uses_forbidden_not_method_not_allowed() {
        assert_eq!(status_for(Method::Get, Route::Admin, true), 200);
        assert_eq!(status_for(Method::Post, Route::Admin, true), 204);
        assert_eq!(status_for(Method::Get, Route::Admin, false), 403);
        assert_eq!(status_for(Method::Post, Route::Admin, false), 403);
        assert_eq!(status_for(Method::Delete, Route::Admin, true), 405);
        assert_eq!(status_for(Method::Delete, Route::Admin, false), 405);
    }
}
