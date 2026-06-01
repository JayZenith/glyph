#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Home,
    Admin,
    Profile,
    Login,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    User,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Render(&'static str),
    Redirect(&'static str),
    Deny,
}

pub fn resolve(route: Route, role: Role, authenticated: bool) -> Action {
    match route {
        Route::Home => Action::Render("home"),
        Route::Admin => match role {
            Role::Admin => Action::Render("admin"),
            _ => Action::Deny,
        },
        Route::Profile => {
            if authenticated {
                Action::Render("profile")
            } else {
                Action::Deny
            }
        }
        Route::Login => {
            if authenticated {
                Action::Render("login")
            } else {
                Action::Render("login")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_is_public() {
        assert_eq!(resolve(Route::Home, Role::Guest, false), Action::Render("home"));
    }

    #[test]
    fn admin_requires_admin_role() {
        assert_eq!(resolve(Route::Admin, Role::Admin, true), Action::Render("admin"));
        assert_eq!(resolve(Route::Admin, Role::User, true), Action::Deny);
        assert_eq!(resolve(Route::Admin, Role::Guest, false), Action::Deny);
    }

    #[test]
    fn profile_requires_authentication() {
        assert_eq!(resolve(Route::Profile, Role::User, true), Action::Render("profile"));
        assert_eq!(resolve(Route::Profile, Role::Guest, false), Action::Redirect("/login"));
    }

    #[test]
    fn login_redirects_authenticated_users_by_role() {
        assert_eq!(resolve(Route::Login, Role::Guest, false), Action::Render("login"));
        assert_eq!(resolve(Route::Login, Role::User, true), Action::Redirect("/profile"));
        assert_eq!(resolve(Route::Login, Role::Admin, true), Action::Redirect("/admin"));
    }
}
