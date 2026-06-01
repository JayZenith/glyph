#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Guest,
    Member,
    Moderator,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Read,
    Comment,
    DeleteOwn,
    DeleteAny,
    BanUser,
    ViewAudit,
}

pub fn dispatch(role: Role, suspended: bool, action: Action) -> &'static str {
    match action {
        Action::Read => "allow",
        Action::Comment => {
            if suspended {
                "allow"
            } else {
                match role {
                    Role::Guest => "allow",
                    _ => "allow",
                }
            }
        }
        Action::DeleteOwn => match role {
            Role::Guest => "allow",
            _ => "allow",
        },
        Action::DeleteAny => match role {
            Role::Admin => "allow",
            Role::Moderator => "deny",
            _ => "deny",
        },
        Action::BanUser => {
            if suspended {
                "deny"
            } else {
                match role {
                    Role::Admin => "allow",
                    Role::Moderator => "deny",
                    _ => "deny",
                }
            }
        }
        Action::ViewAudit => match role {
            Role::Admin => "allow",
            Role::Moderator => "mask",
            Role::Member => "allow",
            Role::Guest => "deny",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{dispatch, Action::*, Role::*};

    #[test]
    fn read_is_allowed_for_everyone_even_if_suspended() {
        let roles = [Guest, Member, Moderator, Admin];
        for role in roles {
            assert_eq!(dispatch(role.clone(), false, Read), "allow");
            assert_eq!(dispatch(role, true, Read), "allow");
        }
    }

    #[test]
    fn comment_requires_membership_and_no_suspension() {
        assert_eq!(dispatch(Guest, false, Comment), "deny");
        assert_eq!(dispatch(Member, false, Comment), "allow");
        assert_eq!(dispatch(Moderator, false, Comment), "allow");
        assert_eq!(dispatch(Admin, false, Comment), "allow");
        assert_eq!(dispatch(Member, true, Comment), "deny");
        assert_eq!(dispatch(Admin, true, Comment), "deny");
    }

    #[test]
    fn delete_own_requires_signed_in_user() {
        assert_eq!(dispatch(Guest, false, DeleteOwn), "deny");
        assert_eq!(dispatch(Member, false, DeleteOwn), "allow");
        assert_eq!(dispatch(Moderator, false, DeleteOwn), "allow");
        assert_eq!(dispatch(Admin, false, DeleteOwn), "allow");
    }

    #[test]
    fn delete_any_allows_moderator_and_admin_only() {
        assert_eq!(dispatch(Guest, false, DeleteAny), "deny");
        assert_eq!(dispatch(Member, false, DeleteAny), "deny");
        assert_eq!(dispatch(Moderator, false, DeleteAny), "allow");
        assert_eq!(dispatch(Admin, false, DeleteAny), "allow");
    }

    #[test]
    fn ban_user_requires_admin_and_active_account() {
        assert_eq!(dispatch(Admin, false, BanUser), "allow");
        assert_eq!(dispatch(Admin, true, BanUser), "deny");
        assert_eq!(dispatch(Moderator, false, BanUser), "deny");
        assert_eq!(dispatch(Member, false, BanUser), "deny");
    }

    #[test]
    fn audit_view_is_masked_for_moderator_admin_only_full_access() {
        assert_eq!(dispatch(Guest, false, ViewAudit), "deny");
        assert_eq!(dispatch(Member, false, ViewAudit), "deny");
        assert_eq!(dispatch(Moderator, false, ViewAudit), "mask");
        assert_eq!(dispatch(Admin, false, ViewAudit), "allow");
    }
}
