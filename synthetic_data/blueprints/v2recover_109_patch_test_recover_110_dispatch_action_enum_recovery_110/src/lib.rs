#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Create { urgent: bool, owner: Option<&'static str> },
    Delete { force: bool, archived: bool },
    Move { from: &'static str, to: &'static str },
    Audit { actor: Option<&'static str>, success: bool },
    Notify { channel: Channel, retries: u8 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    Webhook,
}

pub fn dispatch(action: Action) -> &'static str {
    match action {
        Action::Create { urgent: true, .. } => "create",
        Action::Create { owner: Some(_), .. } => "create:queued",
        Action::Create { .. } => "create:anon",

        Action::Delete { force: true, .. } => "delete:soft",
        Action::Delete { archived: true, .. } => "delete:hard",
        Action::Delete { .. } => "delete",

        Action::Move { from, to } if from == to => "move",
        Action::Move { .. } => "move:rename",

        Action::Audit { actor: None, success: true } => "audit:ok",
        Action::Audit { actor: Some(_), success: false } => "audit:ok",
        Action::Audit { .. } => "audit",

        Action::Notify { channel: Channel::Email, retries } if retries > 0 => "notify:email",
        Action::Notify { channel: Channel::Sms, retries: 0 } => "notify:sms:retry",
        Action::Notify { channel: Channel::Webhook, .. } => "notify:webhook:retry",
        Action::Notify { .. } => "notify",
    }
}

#[cfg(test)]
mod tests {
    use super::{dispatch, Action, Channel};

    #[test]
    fn create_variants() {
        assert_eq!(dispatch(Action::Create { urgent: true, owner: Some("ops") }), "create:urgent");
        assert_eq!(dispatch(Action::Create { urgent: false, owner: Some("ops") }), "create:owned");
        assert_eq!(dispatch(Action::Create { urgent: false, owner: None }), "create:anon");
    }

    #[test]
    fn delete_variants() {
        assert_eq!(dispatch(Action::Delete { force: true, archived: false }), "delete:hard");
        assert_eq!(dispatch(Action::Delete { force: false, archived: true }), "delete:archived");
        assert_eq!(dispatch(Action::Delete { force: false, archived: false }), "delete:soft");
    }

    #[test]
    fn move_variants() {
        assert_eq!(dispatch(Action::Move { from: "a", to: "a" }), "move:noop");
        assert_eq!(dispatch(Action::Move { from: "a", to: "b" }), "move:transfer");
    }

    #[test]
    fn audit_variants() {
        assert_eq!(dispatch(Action::Audit { actor: None, success: true }), "audit:system:ok");
        assert_eq!(dispatch(Action::Audit { actor: Some("sam"), success: false }), "audit:user:fail");
        assert_eq!(dispatch(Action::Audit { actor: Some("sam"), success: true }), "audit:user:ok");
        assert_eq!(dispatch(Action::Audit { actor: None, success: false }), "audit:system:fail");
    }

    #[test]
    fn notify_variants() {
        assert_eq!(dispatch(Action::Notify { channel: Channel::Email, retries: 2 }), "notify:email:retry");
        assert_eq!(dispatch(Action::Notify { channel: Channel::Email, retries: 0 }), "notify:email");
        assert_eq!(dispatch(Action::Notify { channel: Channel::Sms, retries: 0 }), "notify:sms");
        assert_eq!(dispatch(Action::Notify { channel: Channel::Sms, retries: 3 }), "notify:sms:retry");
        assert_eq!(dispatch(Action::Notify { channel: Channel::Webhook, retries: 1 }), "notify:webhook:retry");
        assert_eq!(dispatch(Action::Notify { channel: Channel::Webhook, retries: 0 }), "notify:webhook");
    }
}
