#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    Member,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    View,
    Edit,
    Delete,
    Export,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Own,
    Team,
    Global,
}

pub fn route_for(role: Role, action: Action, scope: Scope, feature_export: bool) -> Result<&'static str, &'static str> {
    match (role, action, scope) {
        (_, Action::View, Scope::Own) => Ok("self_view"),
        (_, Action::View, Scope::Team) => Ok("team_view"),
        (Role::Admin, Action::View, Scope::Global) => Ok("global_view"),
        (Role::Member, Action::Edit, Scope::Own) => Ok("edit_self"),
        (Role::Admin, Action::Edit, Scope::Own | Scope::Team) => Ok("admin_edit"),
        (Role::Admin, Action::Edit, Scope::Global) => Ok("admin_edit"),
        (Role::Member, Action::Delete, Scope::Own) => Ok("delete_self"),
        (Role::Admin, Action::Delete, _) => Ok("delete_any"),
        (_, Action::Export, _) if feature_export => Ok("export_basic"),
        (_, Action::Export, _) => Err("export_disabled"),
        _ => Err("forbidden"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guests_have_only_limited_view_access() {
        assert_eq!(route_for(Role::Guest, Action::View, Scope::Own, false), Ok("self_view"));
        assert_eq!(route_for(Role::Guest, Action::View, Scope::Team, false), Ok("team_view"));
        assert_eq!(route_for(Role::Guest, Action::View, Scope::Global, false), Err("forbidden"));
    }

    #[test]
    fn member_edit_and_delete_are_only_for_own_scope() {
        assert_eq!(route_for(Role::Member, Action::Edit, Scope::Own, false), Ok("edit_self"));
        assert_eq!(route_for(Role::Member, Action::Edit, Scope::Team, false), Err("forbidden"));
        assert_eq!(route_for(Role::Member, Action::Delete, Scope::Own, false), Ok("delete_self"));
        assert_eq!(route_for(Role::Member, Action::Delete, Scope::Team, false), Err("forbidden"));
    }

    #[test]
    fn admin_has_distinct_routes_per_scope() {
        assert_eq!(route_for(Role::Admin, Action::Edit, Scope::Own, false), Ok("admin_edit_self"));
        assert_eq!(route_for(Role::Admin, Action::Edit, Scope::Team, false), Ok("admin_edit_team"));
        assert_eq!(route_for(Role::Admin, Action::Edit, Scope::Global, false), Ok("admin_edit_global"));
        assert_eq!(route_for(Role::Admin, Action::Delete, Scope::Global, false), Ok("delete_any"));
        assert_eq!(route_for(Role::Admin, Action::View, Scope::Global, false), Ok("global_view"));
    }

    #[test]
    fn export_requires_flag_and_role_specific_scope_rules() {
        assert_eq!(route_for(Role::Guest, Action::Export, Scope::Own, false), Err("export_disabled"));
        assert_eq!(route_for(Role::Guest, Action::Export, Scope::Own, true), Err("forbidden"));
        assert_eq!(route_for(Role::Member, Action::Export, Scope::Own, true), Ok("export_own"));
        assert_eq!(route_for(Role::Member, Action::Export, Scope::Team, true), Ok("export_team"));
        assert_eq!(route_for(Role::Member, Action::Export, Scope::Global, true), Err("forbidden"));
        assert_eq!(route_for(Role::Admin, Action::Export, Scope::Global, true), Ok("export_global"));
    }
}
