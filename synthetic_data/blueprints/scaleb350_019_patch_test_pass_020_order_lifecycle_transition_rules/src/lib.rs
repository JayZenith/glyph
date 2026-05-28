#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Draft,
    Submitted,
    Paid,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Pay,
    Ship,
    Deliver,
    Cancel,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Draft;
    for event in events {
        status = match (status, event) {
            (Status::Draft, Event::Submit) => Status::Submitted,
            (Status::Draft, Event::Cancel) => Status::Cancelled,
            (Status::Submitted, Event::Pay) => Status::Paid,
            (Status::Submitted, Event::Cancel) => Status::Cancelled,
            (Status::Paid, Event::Ship) => Status::Shipped,
            (Status::Paid, Event::Cancel) => Status::Cancelled,
            (Status::Shipped, Event::Deliver) => Status::Delivered,
            _ => status,
        };
    }
    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event::*, Status::*};

    #[test]
    fn happy_path_reaches_delivered() {
        assert_eq!(apply_events(&[Submit, Pay, Ship, Deliver]), Delivered);
    }

    #[test]
    fn cancel_before_shipping_is_allowed() {
        assert_eq!(apply_events(&[Submit, Pay, Cancel]), Cancelled);
    }

    #[test]
    fn cancel_after_shipping_is_ignored() {
        assert_eq!(apply_events(&[Submit, Pay, Ship, Cancel]), Shipped);
    }

    #[test]
    fn no_progress_after_terminal_states() {
        assert_eq!(apply_events(&[Cancel, Submit, Pay, Ship, Deliver]), Cancelled);
        assert_eq!(apply_events(&[Submit, Pay, Ship, Deliver, Cancel]), Delivered);
        assert_eq!(apply_events(&[Submit, Pay, Ship, Deliver, Ship]), Delivered);
    }

    #[test]
    fn out_of_order_events_do_not_skip_required_states() {
        assert_eq!(apply_events(&[Pay, Submit, Ship, Deliver]), Submitted);
        assert_eq!(apply_events(&[Submit, Ship, Pay, Deliver]), Paid);
    }
}
