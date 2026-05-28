#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderState {
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

pub fn apply_event(state: OrderState, event: Event) -> OrderState {
    match (state, event) {
        (OrderState::Draft, Event::Submit) => OrderState::Submitted,
        (OrderState::Submitted, Event::Pay) => OrderState::Paid,
        (OrderState::Paid, Event::Ship) => OrderState::Shipped,
        (OrderState::Draft, Event::Cancel)
        | (OrderState::Submitted, Event::Cancel)
        | (OrderState::Paid, Event::Cancel) => OrderState::Cancelled,
        _ => state,
    }
}

pub fn replay(events: &[Event]) -> OrderState {
    events
        .iter()
        .copied()
        .fold(OrderState::Draft, apply_event)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipped_order_stays_shipped_after_late_cancel() {
        let events = [Event::Submit, Event::Pay, Event::Ship, Event::Cancel];
        assert_eq!(replay(&events), OrderState::Shipped);
    }

    #[test]
    fn cancel_before_shipping_works() {
        let events = [Event::Submit, Event::Cancel, Event::Pay, Event::Ship];
        assert_eq!(replay(&events), OrderState::Cancelled);
    }

    #[test]
    fn invalid_events_do_not_advance_state() {
        let events = [Event::Pay, Event::Ship, Event::Submit, Event::Ship, Event::Pay];
        assert_eq!(replay(&events), OrderState::Paid);
    }

    #[test]
    fn draft_can_be_cancelled_directly() {
        assert_eq!(replay(&[Event::Cancel]), OrderState::Cancelled);
    }
}
