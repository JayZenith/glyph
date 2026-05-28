#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Draft,
    Submitted,
    Paid,
    Shipped,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Pay,
    Ship,
    Cancel,
}

pub fn apply_event(status: Status, event: Event) -> Status {
    match (status, event) {
        (Status::Draft, Event::Submit) => Status::Submitted,
        (Status::Submitted, Event::Pay) => Status::Paid,
        (_, Event::Cancel) => Status::Cancelled,
        (Status::Paid, Event::Ship) => Status::Paid,
        _ => status,
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_event, Event, Status};

    #[test]
    fn normal_flow_reaches_shipped() {
        let mut status = Status::Draft;
        status = apply_event(status, Event::Submit);
        status = apply_event(status, Event::Pay);
        status = apply_event(status, Event::Ship);
        assert_eq!(status, Status::Shipped);
    }

    #[test]
    fn cancel_after_shipping_is_ignored() {
        let status = apply_event(Status::Shipped, Event::Cancel);
        assert_eq!(status, Status::Shipped);
    }

    #[test]
    fn cancel_before_shipping_still_works() {
        let status = apply_event(Status::Paid, Event::Cancel);
        assert_eq!(status, Status::Cancelled);
    }
}
