#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Open,
    InProgress,
    Blocked,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Block,
    Unblock,
    Resolve,
    Close,
    Reopen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub state: State,
    pub assignee: Option<&'static str>,
    pub resolution: Option<&'static str>,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            state: State::Open,
            assignee: None,
            resolution: None,
        }
    }

    pub fn apply(&mut self, event: Event) -> bool {
        match event {
            Event::Start => {
                self.state = State::InProgress;
                true
            }
            Event::Block => {
                self.state = State::Blocked;
                true
            }
            Event::Unblock => {
                self.state = State::Open;
                true
            }
            Event::Resolve => {
                self.state = State::Resolved;
                true
            }
            Event::Close => {
                self.state = State::Closed;
                self.resolution = None;
                true
            }
            Event::Reopen => {
                self.state = State::Open;
                true
            }
        }
    }

    pub fn assign(&mut self, who: &'static str) {
        self.assignee = Some(who);
    }

    pub fn set_resolution(&mut self, resolution: &'static str) {
        self.resolution = Some(resolution);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_flow_preserves_data_and_obeys_transition_rules() {
        let mut t = Ticket::new();
        t.assign("alex");

        assert!(t.apply(Event::Start));
        assert_eq!(t.state, State::InProgress);
        assert_eq!(t.assignee, Some("alex"));

        assert!(t.apply(Event::Block));
        assert_eq!(t.state, State::Blocked);

        assert!(t.apply(Event::Unblock));
        assert_eq!(t.state, State::InProgress);

        t.set_resolution("fixed");
        assert!(t.apply(Event::Resolve));
        assert_eq!(t.state, State::Resolved);
        assert_eq!(t.resolution, Some("fixed"));

        assert!(t.apply(Event::Close));
        assert_eq!(t.state, State::Closed);
        assert_eq!(t.assignee, Some("alex"));
        assert_eq!(t.resolution, Some("fixed"));
    }

    #[test]
    fn invalid_transitions_are_rejected_and_do_not_mutate() {
        let mut t = Ticket::new();
        t.assign("sam");
        t.set_resolution("fixed");

        assert!(!t.apply(Event::Close));
        assert_eq!(t.state, State::Open);
        assert_eq!(t.assignee, Some("sam"));
        assert_eq!(t.resolution, Some("fixed"));

        assert!(!t.apply(Event::Resolve));
        assert_eq!(t.state, State::Open);

        assert!(t.apply(Event::Start));
        assert!(!t.apply(Event::Unblock));
        assert_eq!(t.state, State::InProgress);

        assert!(t.apply(Event::Block));
        assert!(!t.apply(Event::Start));
        assert_eq!(t.state, State::Blocked);
    }

    #[test]
    fn reopen_clears_resolution_but_keeps_assignee() {
        let mut t = Ticket::new();
        t.assign("jules");
        assert!(t.apply(Event::Start));
        t.set_resolution("won't fix");
        assert!(t.apply(Event::Resolve));
        assert!(t.apply(Event::Close));

        assert!(t.apply(Event::Reopen));
        assert_eq!(t.state, State::Open);
        assert_eq!(t.assignee, Some("jules"));
        assert_eq!(t.resolution, None);

        assert!(t.apply(Event::Start));
        assert!(t.apply(Event::Resolve));
        assert!(t.apply(Event::Close));
    }
}
