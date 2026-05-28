#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipmentState {
    Preparing,
    Packed,
    InTransit,
    Delivered,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    PrintLabel,
    Dispatch,
    ConfirmDelivery,
    OpenInvestigation,
}

pub fn next_state(state: ShipmentState, action: Action) -> ShipmentState {
    match action {
        Action::PrintLabel => ShipmentState::Packed,
        Action::Dispatch => match state {
            ShipmentState::Packed => ShipmentState::InTransit,
            _ => state,
        },
        Action::ConfirmDelivery => match state {
            ShipmentState::InTransit => ShipmentState::Delivered,
            ShipmentState::Lost => ShipmentState::Delivered,
            _ => state,
        },
        Action::OpenInvestigation => match state {
            ShipmentState::InTransit => ShipmentState::Lost,
            _ => state,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printing_label_only_advances_preparing_shipments() {
        assert_eq!(next_state(ShipmentState::Preparing, Action::PrintLabel), ShipmentState::Packed);
        assert_eq!(next_state(ShipmentState::Packed, Action::PrintLabel), ShipmentState::Packed);
        assert_eq!(next_state(ShipmentState::Delivered, Action::PrintLabel), ShipmentState::Delivered);
    }

    #[test]
    fn dispatch_requires_packed_state() {
        assert_eq!(next_state(ShipmentState::Packed, Action::Dispatch), ShipmentState::InTransit);
        assert_eq!(next_state(ShipmentState::Preparing, Action::Dispatch), ShipmentState::Preparing);
    }

    #[test]
    fn delivery_confirmation_only_works_for_in_transit() {
        assert_eq!(next_state(ShipmentState::InTransit, Action::ConfirmDelivery), ShipmentState::Delivered);
        assert_eq!(next_state(ShipmentState::Lost, Action::ConfirmDelivery), ShipmentState::Lost);
        assert_eq!(next_state(ShipmentState::Packed, Action::ConfirmDelivery), ShipmentState::Packed);
    }

    #[test]
    fn investigation_marks_in_transit_shipments_as_lost() {
        assert_eq!(next_state(ShipmentState::InTransit, Action::OpenInvestigation), ShipmentState::Lost);
        assert_eq!(next_state(ShipmentState::Delivered, Action::OpenInvestigation), ShipmentState::Delivered);
    }
}
