#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Allow,
    Deny,
    Audit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Api,
    User,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Login,
    Data,
    Admin,
    Health,
}

pub fn decide(route: Route, source: Source, authenticated: bool) -> Action {
    match route {
        Route::Health => Action::Deny,
        Route::Login => match source {
            Source::Api => Action::Allow,
            Source::User => Action::Allow,
            Source::System => Action::Allow,
        },
        Route::Data => {
            if authenticated {
                Action::Allow
            } else {
                Action::Audit
            }
        }
        Route::Admin => match source {
            Source::System => Action::Allow,
            _ if authenticated => Action::Audit,
            _ => Action::Deny,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_is_always_allowed() {
        assert_eq!(decide(Route::Health, Source::Api, false), Action::Allow);
        assert_eq!(decide(Route::Health, Source::User, true), Action::Allow);
        assert_eq!(decide(Route::Health, Source::System, false), Action::Allow);
    }

    #[test]
    fn login_is_not_open_to_system_and_needs_auth_for_users() {
        assert_eq!(decide(Route::Login, Source::Api, false), Action::Allow);
        assert_eq!(decide(Route::Login, Source::User, false), Action::Audit);
        assert_eq!(decide(Route::Login, Source::User, true), Action::Allow);
        assert_eq!(decide(Route::Login, Source::System, true), Action::Deny);
    }

    #[test]
    fn data_requires_authentication_for_every_source() {
        assert_eq!(decide(Route::Data, Source::Api, false), Action::Deny);
        assert_eq!(decide(Route::Data, Source::User, false), Action::Deny);
        assert_eq!(decide(Route::Data, Source::System, false), Action::Deny);
        assert_eq!(decide(Route::Data, Source::Api, true), Action::Allow);
        assert_eq!(decide(Route::Data, Source::User, true), Action::Allow);
        assert_eq!(decide(Route::Data, Source::System, true), Action::Allow);
    }

    #[test]
    fn admin_only_allows_authenticated_system_and_denies_the_rest() {
        assert_eq!(decide(Route::Admin, Source::System, true), Action::Allow);
        assert_eq!(decide(Route::Admin, Source::System, false), Action::Deny);
        assert_eq!(decide(Route::Admin, Source::Api, true), Action::Deny);
        assert_eq!(decide(Route::Admin, Source::User, true), Action::Deny);
        assert_eq!(decide(Route::Admin, Source::Api, false), Action::Deny);
        assert_eq!(decide(Route::Admin, Source::User, false), Action::Deny);
    }
}
