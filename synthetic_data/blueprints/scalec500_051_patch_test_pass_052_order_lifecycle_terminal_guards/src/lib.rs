#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Draft,
    Submitted,
    Approved,
    Shipped,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Approve,
    Ship,
    Cancel,
}

pub fn apply_event(status: Status, event: Event) -> Status {
    match event {
        Event::Submit => Status::Submitted,
        Event::Approve => match status {
            Status::Submitted => Status::Approved,
            _ => status,
        },
        Event::Ship => match status {
            Status::Approved => Status::Shipped,
            _ => status,
        },
        Event::Cancel => Status::Cancelled,
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_event, Event, Status};

    #[test]
    fn happy_path_reaches_shipped() {
        let s = apply_event(Status::Draft, Event::Submit);
        let s = apply_event(s, Event::Approve);
        let s = apply_event(s, Event::Ship);
        assert_eq!(s, Status::Shipped);
    }

    #[test]
    fn cannot_approve_before_submit() {
        assert_eq!(apply_event(Status::Draft, Event::Approve), Status::Draft);
    }

    #[test]
    fn cannot_ship_before_approval() {
        assert_eq!(apply_event(Status::Submitted, Event::Ship), Status::Submitted);
    }

    #[test]
    fn cancelled_is_terminal() {
        let s = apply_event(Status::Approved, Event::Cancel);
        assert_eq!(s, Status::Cancelled);
        assert_eq!(apply_event(s, Event::Submit), Status::Cancelled);
        assert_eq!(apply_event(s, Event::Approve), Status::Cancelled);
        assert_eq!(apply_event(s, Event::Ship), Status::Cancelled);
    }

    #[test]
    fn shipped_is_terminal() {
        let s = Status::Shipped;
        assert_eq!(apply_event(s, Event::Cancel), Status::Shipped);
        assert_eq!(apply_event(s, Event::Submit), Status::Shipped);
    }
}
