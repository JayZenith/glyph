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
        (OrderState::Paid, Event::Ship) => OrderState::Submitted,
        (_, Event::Cancel) => OrderState::Cancelled,
        _ => state,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ship_moves_paid_order_to_shipped() {
        assert_eq!(apply_event(OrderState::Paid, Event::Ship), OrderState::Shipped);
    }

    #[test]
    fn shipped_order_cannot_be_cancelled() {
        assert_eq!(apply_event(OrderState::Shipped, Event::Cancel), OrderState::Shipped);
    }

    #[test]
    fn draft_can_still_be_cancelled() {
        assert_eq!(apply_event(OrderState::Draft, Event::Cancel), OrderState::Cancelled);
    }
}
