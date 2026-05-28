#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    Admin,
    Member,
    Guest,
    Suspended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Dashboard,
    Settings,
    Billing,
    Login,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Allow,
    Redirect(Route),
    Deny(&'static str),
}

pub fn decide(access: Access, route: Route, has_2fa: bool, billing_ok: bool) -> Action {
    match route {
        Route::Login => Action::Allow,
        Route::Help => Action::Allow,
        Route::Dashboard => match access {
            Access::Admin | Access::Member => Action::Allow,
            Access::Guest => Action::Redirect(Route::Login),
            Access::Suspended => Action::Deny("suspended"),
        },
        Route::Settings => match access {
            Access::Admin => {
                if has_2fa {
                    Action::Allow
                } else {
                    Action::Redirect(Route::Help)
                }
            }
            Access::Member => Action::Allow,
            Access::Guest => Action::Redirect(Route::Login),
            Access::Suspended => Action::Deny("suspended"),
        },
        Route::Billing => match access {
            Access::Admin => Action::Allow,
            Access::Member => {
                if billing_ok {
                    Action::Allow
                } else {
                    Action::Redirect(Route::Dashboard)
                }
            }
            Access::Guest => Action::Redirect(Route::Login),
            Access::Suspended => Action::Redirect(Route::Login),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_settings_requires_2fa() {
        assert_eq!(
            decide(Access::Admin, Route::Settings, false, true),
            Action::Redirect(Route::Login)
        );
        assert_eq!(
            decide(Access::Admin, Route::Settings, true, true),
            Action::Allow
        );
    }

    #[test]
    fn member_settings_need_not_have_2fa() {
        assert_eq!(
            decide(Access::Member, Route::Settings, false, true),
            Action::Allow
        );
    }

    #[test]
    fn member_billing_without_account_goes_to_help() {
        assert_eq!(
            decide(Access::Member, Route::Billing, true, false),
            Action::Redirect(Route::Help)
        );
    }

    #[test]
    fn suspended_users_are_denied_everywhere_except_help_and_login() {
        assert_eq!(
            decide(Access::Suspended, Route::Dashboard, true, true),
            Action::Deny("suspended")
        );
        assert_eq!(
            decide(Access::Suspended, Route::Settings, true, true),
            Action::Deny("suspended")
        );
        assert_eq!(
            decide(Access::Suspended, Route::Billing, true, true),
            Action::Deny("suspended")
        );
        assert_eq!(
            decide(Access::Suspended, Route::Help, true, true),
            Action::Allow
        );
        assert_eq!(
            decide(Access::Suspended, Route::Login, true, true),
            Action::Allow
        );
    }

    #[test]
    fn guests_can_only_reach_public_routes() {
        assert_eq!(
            decide(Access::Guest, Route::Dashboard, false, false),
            Action::Redirect(Route::Login)
        );
        assert_eq!(
            decide(Access::Guest, Route::Billing, false, false),
            Action::Redirect(Route::Login)
        );
        assert_eq!(
            decide(Access::Guest, Route::Help, false, false),
            Action::Allow
        );
    }
}
