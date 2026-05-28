#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Home,
    Login,
    Admin,
    Project { archived: bool },
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum User {
    Guest,
    Member,
    Admin,
}

pub fn action_for(route: Route, user: User) -> &'static str {
    match route {
        Route::Home => "show_home",
        Route::Login => match user {
            User::Guest => "prompt_login",
            _ => "show_home",
        },
        Route::Admin => match user {
            User::Admin => "open_admin",
            _ => "forbidden",
        },
        Route::Project { archived } => match user {
            User::Guest => "prompt_login",
            User::Member if archived => "edit_project",
            User::Member => "edit_project",
            User::Admin => "edit_project",
        },
        Route::Missing => "redirect_home",
    }
}

#[cfg(test)]
mod tests {
    use super::{action_for, Route, User};

    #[test]
    fn guest_must_login_for_project() {
        assert_eq!(action_for(Route::Project { archived: false }, User::Guest), "prompt_login");
    }

    #[test]
    fn member_can_edit_active_project() {
        assert_eq!(action_for(Route::Project { archived: false }, User::Member), "edit_project");
    }

    #[test]
    fn member_gets_read_only_for_archived_project() {
        assert_eq!(action_for(Route::Project { archived: true }, User::Member), "view_project");
    }

    #[test]
    fn admin_can_edit_archived_project() {
        assert_eq!(action_for(Route::Project { archived: true }, User::Admin), "edit_project");
    }

    #[test]
    fn logged_in_user_visiting_login_goes_home() {
        assert_eq!(action_for(Route::Login, User::Member), "show_home");
    }

    #[test]
    fn missing_route_redirects_home() {
        assert_eq!(action_for(Route::Missing, User::Guest), "redirect_home");
    }
}
