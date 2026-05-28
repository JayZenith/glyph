#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Allow,
    Deny,
    Redirect(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteKind {
    Exact,
    Prefix,
    Fallback,
}

pub fn decide_action(kind: RouteKind, action: Action, authenticated: bool) -> String {
    match (kind, action, authenticated) {
        (RouteKind::Exact, Action::Allow, true) => "allow".to_string(),
        (RouteKind::Exact, Action::Allow, false) => "deny".to_string(),
        (RouteKind::Exact, Action::Deny, _) => "deny".to_string(),
        (RouteKind::Exact, Action::Redirect(path), _) => format!("redirect:{path}"),
        (RouteKind::Prefix, Action::Allow, _) => "allow".to_string(),
        (RouteKind::Prefix, Action::Deny, _) => "deny".to_string(),
        (RouteKind::Prefix, Action::Redirect(path), _) => format!("redirect:{path}"),
        (RouteKind::Fallback, Action::Allow, _) => "allow".to_string(),
        (RouteKind::Fallback, Action::Deny, _) => "deny".to_string(),
        (RouteKind::Fallback, Action::Redirect(_), _) => "deny".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_allow_requires_auth() {
        assert_eq!(decide_action(RouteKind::Exact, Action::Allow, true), "allow");
        assert_eq!(decide_action(RouteKind::Exact, Action::Allow, false), "deny");
    }

    #[test]
    fn prefix_redirect_keeps_target() {
        assert_eq!(
            decide_action(RouteKind::Prefix, Action::Redirect("/docs"), false),
            "redirect:/docs"
        );
    }

    #[test]
    fn fallback_allow_is_guarded_but_redirects_are_preserved() {
        assert_eq!(decide_action(RouteKind::Fallback, Action::Allow, true), "allow");
        assert_eq!(decide_action(RouteKind::Fallback, Action::Allow, false), "deny");
        assert_eq!(
            decide_action(RouteKind::Fallback, Action::Redirect("/login"), false),
            "redirect:/login"
        );
    }
}
