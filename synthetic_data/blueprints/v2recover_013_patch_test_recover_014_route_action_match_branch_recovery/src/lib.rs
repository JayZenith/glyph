#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Home,
    Search,
    Account,
    Admin,
    Asset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    User,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Render(&'static str),
    Redirect(&'static str),
    Json(&'static str),
    Deny(u16),
}

pub fn plan(route: Route, role: Role, method: Method, secure: bool) -> Action {
    match route {
        Route::Home => Action::Render("home"),
        Route::Search => match method {
            Method::Get => Action::Render("search"),
            _ => Action::Deny(405),
        },
        Route::Account => match (role, method, secure) {
            (Role::Guest, _, _) => Action::Redirect("/login"),
            (_, Method::Get, false) => Action::Render("account"),
            (_, Method::Get, true) => Action::Render("account"),
            (_, Method::Post, _) => Action::Json("saved"),
            _ => Action::Deny(405),
        },
        Route::Admin => match (role, method) {
            (Role::Admin, Method::Get) => Action::Render("admin"),
            (Role::Admin, Method::Post) => Action::Json("ok"),
            _ => Action::Redirect("/login"),
        },
        Route::Asset => match method {
            Method::Get => Action::Render("asset"),
            _ => Action::Deny(404),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_always_renders() {
        assert_eq!(plan(Route::Home, Role::Guest, Method::Get, false), Action::Render("home"));
        assert_eq!(plan(Route::Home, Role::Admin, Method::Delete, true), Action::Render("home"));
    }

    #[test]
    fn search_only_allows_get() {
        assert_eq!(plan(Route::Search, Role::User, Method::Get, false), Action::Render("search"));
        assert_eq!(plan(Route::Search, Role::User, Method::Post, false), Action::Deny(405));
    }

    #[test]
    fn account_requires_login_and_https() {
        assert_eq!(plan(Route::Account, Role::Guest, Method::Get, true), Action::Redirect("/login"));
        assert_eq!(plan(Route::Account, Role::User, Method::Get, false), Action::Redirect("https://account"));
        assert_eq!(plan(Route::Account, Role::User, Method::Get, true), Action::Render("account"));
    }

    #[test]
    fn account_post_requires_https_and_user() {
        assert_eq!(plan(Route::Account, Role::User, Method::Post, false), Action::Redirect("https://account"));
        assert_eq!(plan(Route::Account, Role::User, Method::Post, true), Action::Json("saved"));
        assert_eq!(plan(Route::Account, Role::Admin, Method::Post, true), Action::Json("saved"));
    }

    #[test]
    fn admin_requires_admin_role() {
        assert_eq!(plan(Route::Admin, Role::Guest, Method::Get, true), Action::Deny(403));
        assert_eq!(plan(Route::Admin, Role::User, Method::Post, true), Action::Deny(403));
        assert_eq!(plan(Route::Admin, Role::Admin, Method::Get, true), Action::Render("admin"));
    }

    #[test]
    fn admin_delete_is_not_allowed_even_for_admin() {
        assert_eq!(plan(Route::Admin, Role::Admin, Method::Delete, true), Action::Deny(405));
    }

    #[test]
    fn assets_are_json_only_for_get() {
        assert_eq!(plan(Route::Asset, Role::Guest, Method::Get, false), Action::Json("asset"));
        assert_eq!(plan(Route::Asset, Role::Guest, Method::Delete, false), Action::Deny(405));
    }
}
