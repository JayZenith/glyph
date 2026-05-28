#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    Active,
    Waiting,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Assign,
    CustomerReply,
    AgentReply,
    Resolve,
    Close,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::New;
    let mut had_resolution = false;

    for event in events {
        match event {
            Event::Assign => {
                if status == Status::New {
                    status = Status::Active;
                }
            }
            Event::CustomerReply => {
                status = Status::Active;
            }
            Event::AgentReply => {
                if status == Status::Active {
                    status = Status::Waiting;
                }
            }
            Event::Resolve => {
                if status == Status::Active || status == Status::Waiting {
                    status = Status::Resolved;
                    had_resolution = true;
                }
            }
            Event::Close => {
                if status == Status::Resolved {
                    status = Status::Closed;
                }
            }
            Event::Reopen => {
                if had_resolution {
                    status = Status::Active;
                }
            }
        }
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event, Status};

    #[test]
    fn happy_path_can_close_after_resolution() {
        let events = [Event::Assign, Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn agent_reply_puts_active_ticket_into_waiting() {
        let events = [Event::Assign, Event::AgentReply];
        assert_eq!(apply_events(&events), Status::Waiting);
    }

    #[test]
    fn customer_reply_reopens_waiting_ticket_to_active() {
        let events = [Event::Assign, Event::AgentReply, Event::CustomerReply];
        assert_eq!(apply_events(&events), Status::Active);
    }

    #[test]
    fn reopen_only_works_from_resolved_or_closed() {
        let events = [Event::Assign, Event::AgentReply, Event::Reopen];
        assert_eq!(apply_events(&events), Status::Waiting);
    }

    #[test]
    fn reopen_after_closing_returns_to_active() {
        let events = [
            Event::Assign,
            Event::Resolve,
            Event::Close,
            Event::Reopen,
        ];
        assert_eq!(apply_events(&events), Status::Active);
    }

    #[test]
    fn customer_reply_after_resolution_reopens_to_active() {
        let events = [Event::Assign, Event::Resolve, Event::CustomerReply];
        assert_eq!(apply_events(&events), Status::Active);
    }

    #[test]
    fn assign_does_not_reactivate_closed_ticket() {
        let events = [
            Event::Assign,
            Event::Resolve,
            Event::Close,
            Event::Assign,
        ];
        assert_eq!(apply_events(&events), Status::Closed);
    }

    #[test]
    fn resolve_from_new_is_ignored() {
        let events = [Event::Resolve, Event::Close];
        assert_eq!(apply_events(&events), Status::New);
    }
}
