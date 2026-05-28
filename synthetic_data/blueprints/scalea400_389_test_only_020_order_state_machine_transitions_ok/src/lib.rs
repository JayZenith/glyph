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
    use Event::*;
    use OrderState::*;

    match (state, event) {
        (Draft, Submit) => Submitted,
        (Draft, Cancel) => Cancelled,
        (Submitted, Pay) => Paid,
        (Submitted, Cancel) => Cancelled,
        (Paid, Ship) => Shipped,
        (s, _) => s,
    }
}

pub fn apply_events(mut state: OrderState, events: &[Event]) -> OrderState {
    for &event in events {
        state = apply_event(state, event);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progresses_through_happy_path() {
        let final_state = apply_events(
            OrderState::Draft,
            &[Event::Submit, Event::Pay, Event::Ship],
        );
        assert_eq!(final_state, OrderState::Shipped);
    }

    #[test]
    fn can_cancel_before_shipping() {
        assert_eq!(apply_event(OrderState::Draft, Event::Cancel), OrderState::Cancelled);
        assert_eq!(apply_event(OrderState::Submitted, Event::Cancel), OrderState::Cancelled);
    }

    #[test]
    fn shipped_order_is_terminal() {
        assert_eq!(apply_event(OrderState::Shipped, Event::Cancel), OrderState::Shipped);
        assert_eq!(apply_event(OrderState::Shipped, Event::Pay), OrderState::Shipped);
    }

    #[test]
    fn ignores_invalid_transitions() {
        assert_eq!(apply_event(OrderState::Draft, Event::Ship), OrderState::Draft);
        assert_eq!(apply_event(OrderState::Paid, Event::Submit), OrderState::Paid);
        assert_eq!(apply_event(OrderState::Cancelled, Event::Pay), OrderState::Cancelled);
    }
}
