#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Blocked,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub status: Status,
    pub assignee: Option<String>,
    pub resolution: Option<String>,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::Open,
            assignee: None,
            resolution: None,
        }
    }
}

pub fn apply_event(ticket: &mut Ticket, event: &str) {
    match event {
        "assign" => {
            ticket.assignee = Some("worker".to_string());
        }
        "start" => {
            ticket.status = Status::InProgress;
        }
        "block" => {
            ticket.status = Status::Blocked;
        }
        "resume" => {
            ticket.status = Status::InProgress;
        }
        "close" => {
            ticket.status = Status::Closed;
            ticket.resolution = Some("done".to_string());
        }
        "reopen" => {
            ticket.status = Status::Open;
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_start_without_assignee() {
        let mut t = Ticket::new();
        apply_event(&mut t, "start");
        assert_eq!(t.status, Status::Open);
    }

    #[test]
    fn block_only_from_in_progress() {
        let mut t = Ticket::new();
        apply_event(&mut t, "assign");
        apply_event(&mut t, "block");
        assert_eq!(t.status, Status::Open);
        apply_event(&mut t, "start");
        apply_event(&mut t, "block");
        assert_eq!(t.status, Status::Blocked);
    }

    #[test]
    fn close_requires_in_progress_and_keeps_assignee() {
        let mut t = Ticket::new();
        apply_event(&mut t, "assign");
        apply_event(&mut t, "close");
        assert_eq!(t.status, Status::Open);
        assert_eq!(t.resolution, None);

        apply_event(&mut t, "start");
        apply_event(&mut t, "close");
        assert_eq!(t.status, Status::Closed);
        assert_eq!(t.assignee.as_deref(), Some("worker"));
        assert_eq!(t.resolution.as_deref(), Some("done"));
    }

    #[test]
    fn reopen_clears_resolution_but_preserves_assignee() {
        let mut t = Ticket::new();
        apply_event(&mut t, "assign");
        apply_event(&mut t, "start");
        apply_event(&mut t, "close");
        apply_event(&mut t, "reopen");
        assert_eq!(t.status, Status::Open);
        assert_eq!(t.assignee.as_deref(), Some("worker"));
        assert_eq!(t.resolution, None);
    }
}
