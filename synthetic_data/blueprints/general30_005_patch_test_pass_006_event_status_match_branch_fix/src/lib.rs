#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Create,
    Update,
    Delete,
    Archive,
}

pub fn is_terminal(event: Event, soft_delete: bool) -> bool {
    match event {
        Event::Create | Event::Update => false,
        Event::Delete => false,
        Event::Archive => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_update_are_not_terminal() {
        assert!(!is_terminal(Event::Create, false));
        assert!(!is_terminal(Event::Update, true));
    }

    #[test]
    fn archive_is_always_terminal() {
        assert!(is_terminal(Event::Archive, false));
        assert!(is_terminal(Event::Archive, true));
    }

    #[test]
    fn hard_delete_is_terminal_but_soft_delete_is_not() {
        assert!(!is_terminal(Event::Delete, true));
        assert!(is_terminal(Event::Delete, false));
    }
}
