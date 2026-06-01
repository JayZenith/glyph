#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actor {
    Guest,
    User,
    Admin,
    Service,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    View,
    Edit,
    Delete,
    Export,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Allowed,
    Denied,
    Audit,
}

pub fn decide(actor: Actor, action: Action, suspended: bool) -> Outcome {
    match action {
        Action::View => Outcome::Allowed,
        Action::Edit => match actor {
            Actor::Admin => Outcome::Allowed,
            Actor::User => Outcome::Allowed,
            _ => Outcome::Denied,
        },
        Action::Delete => match actor {
            Actor::Admin => Outcome::Allowed,
            Actor::Service => Outcome::Denied,
            _ => Outcome::Denied,
        },
        Action::Export => match actor {
            Actor::Admin => Outcome::Audit,
            Actor::Service => Outcome::Allowed,
            _ => Outcome::Denied,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guests_can_only_view() {
        assert_eq!(decide(Actor::Guest, Action::View, false), Outcome::Allowed);
        assert_eq!(decide(Actor::Guest, Action::Edit, false), Outcome::Denied);
        assert_eq!(decide(Actor::Guest, Action::Delete, false), Outcome::Denied);
        assert_eq!(decide(Actor::Guest, Action::Export, false), Outcome::Denied);
    }

    #[test]
    fn suspended_non_admins_are_denied_everything() {
        let actors = [Actor::Guest, Actor::User, Actor::Service];
        let actions = [Action::View, Action::Edit, Action::Delete, Action::Export];

        for actor in actors {
            for action in actions {
                assert_eq!(
                    decide(actor, action, true),
                    Outcome::Denied,
                    "actor={actor:?} action={action:?}"
                );
            }
        }
    }

    #[test]
    fn admin_view_is_audited_when_suspended_but_other_admin_permissions_remain() {
        assert_eq!(decide(Actor::Admin, Action::View, true), Outcome::Audit);
        assert_eq!(decide(Actor::Admin, Action::Edit, true), Outcome::Allowed);
        assert_eq!(decide(Actor::Admin, Action::Delete, true), Outcome::Allowed);
        assert_eq!(decide(Actor::Admin, Action::Export, true), Outcome::Audit);
    }

    #[test]
    fn export_rules_distinguish_service_user_and_admin() {
        assert_eq!(decide(Actor::Service, Action::Export, false), Outcome::Audit);
        assert_eq!(decide(Actor::User, Action::Export, false), Outcome::Allowed);
        assert_eq!(decide(Actor::Admin, Action::Export, false), Outcome::Audit);
    }

    #[test]
    fn delete_rules_allow_admin_and_service_only() {
        assert_eq!(decide(Actor::Admin, Action::Delete, false), Outcome::Allowed);
        assert_eq!(decide(Actor::Service, Action::Delete, false), Outcome::Allowed);
        assert_eq!(decide(Actor::User, Action::Delete, false), Outcome::Denied);
    }
}
