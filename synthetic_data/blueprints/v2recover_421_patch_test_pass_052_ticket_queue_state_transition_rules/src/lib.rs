use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketState {
    New,
    InProgress,
    Blocked,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub state: TicketState,
    pub assignee: Option<String>,
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Assign(&'static str),
    StartWork,
    Block,
    Unblock,
    Resolve(&'static str),
    Close,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> Ticket {
    let mut ticket = Ticket {
        state: TicketState::New,
        assignee: None,
        resolution: None,
    };

    for event in events {
        match *event {
            Event::Assign(name) => {
                ticket.assignee = Some(name.to_string());
            }
            Event::StartWork => {
                ticket.state = TicketState::InProgress;
            }
            Event::Block => {
                ticket.state = TicketState::Blocked;
            }
            Event::Unblock => {
                ticket.state = TicketState::InProgress;
            }
            Event::Resolve(note) => {
                ticket.state = TicketState::Resolved;
                ticket.resolution = Some(note.to_string());
            }
            Event::Close => {
                ticket.state = TicketState::Closed;
            }
            Event::Reopen => {
                ticket.state = TicketState::InProgress;
            }
        }
    }

    ticket
}

pub fn summarize(events: &[Event]) -> HashMap<&'static str, usize> {
    let mut counts = HashMap::new();
    for event in events {
        let key = match event {
            Event::Assign(_) => "assign",
            Event::StartWork => "start",
            Event::Block => "block",
            Event::Unblock => "unblock",
            Event::Resolve(_) => "resolve",
            Event::Close => "close",
            Event::Reopen => "reopen",
        };
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assignment_alone_keeps_ticket_new() {
        let ticket = apply_events(&[Event::Assign("dana")]);
        assert_eq!(ticket.state, TicketState::New);
        assert_eq!(ticket.assignee.as_deref(), Some("dana"));
        assert_eq!(ticket.resolution, None);
    }

    #[test]
    fn work_cannot_start_without_assignee() {
        let ticket = apply_events(&[Event::StartWork]);
        assert_eq!(ticket.state, TicketState::New);
        assert_eq!(ticket.assignee, None);
    }

    #[test]
    fn resolve_requires_in_progress_and_keeps_note_when_valid() {
        let invalid = apply_events(&[Event::Assign("dana"), Event::Resolve("done")]);
        assert_eq!(invalid.state, TicketState::New);
        assert_eq!(invalid.resolution, None);

        let valid = apply_events(&[
            Event::Assign("dana"),
            Event::StartWork,
            Event::Resolve("done"),
        ]);
        assert_eq!(valid.state, TicketState::Resolved);
        assert_eq!(valid.resolution.as_deref(), Some("done"));
    }

    #[test]
    fn unblock_only_changes_blocked_ticket() {
        let untouched = apply_events(&[
            Event::Assign("dana"),
            Event::StartWork,
            Event::Unblock,
        ]);
        assert_eq!(untouched.state, TicketState::InProgress);

        let blocked = apply_events(&[
            Event::Assign("dana"),
            Event::StartWork,
            Event::Block,
            Event::Unblock,
        ]);
        assert_eq!(blocked.state, TicketState::InProgress);
    }

    #[test]
    fn close_requires_resolved_and_reopen_clears_resolution() {
        let not_closed = apply_events(&[
            Event::Assign("dana"),
            Event::StartWork,
            Event::Close,
        ]);
        assert_eq!(not_closed.state, TicketState::InProgress);

        let reopened = apply_events(&[
            Event::Assign("dana"),
            Event::StartWork,
            Event::Resolve("fixed"),
            Event::Close,
            Event::Reopen,
        ]);
        assert_eq!(reopened.state, TicketState::InProgress);
        assert_eq!(reopened.assignee.as_deref(), Some("dana"));
        assert_eq!(reopened.resolution, None);
    }

    #[test]
    fn blocked_ticket_cannot_resolve_until_unblocked() {
        let ticket = apply_events(&[
            Event::Assign("dana"),
            Event::StartWork,
            Event::Block,
            Event::Resolve("fixed"),
            Event::Unblock,
            Event::Resolve("fixed"),
        ]);
        assert_eq!(ticket.state, TicketState::Resolved);
        assert_eq!(ticket.resolution.as_deref(), Some("fixed"));
    }

    #[test]
    fn summarize_counts_event_kinds() {
        let counts = summarize(&[
            Event::Assign("dana"),
            Event::StartWork,
            Event::Block,
            Event::Unblock,
            Event::Resolve("fixed"),
            Event::Close,
            Event::Reopen,
        ]);
        assert_eq!(counts.get("assign"), Some(&1));
        assert_eq!(counts.get("start"), Some(&1));
        assert_eq!(counts.get("block"), Some(&1));
        assert_eq!(counts.get("unblock"), Some(&1));
        assert_eq!(counts.get("resolve"), Some(&1));
        assert_eq!(counts.get("close"), Some(&1));
        assert_eq!(counts.get("reopen"), Some(&1));
    }
}
