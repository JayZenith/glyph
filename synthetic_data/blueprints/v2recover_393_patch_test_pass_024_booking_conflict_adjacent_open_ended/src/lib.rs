#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Booking {
    pub start: u32,
    pub end: Option<u32>,
}

impl Booking {
    pub fn new(start: u32, end: Option<u32>) -> Self {
        Self { start, end }
    }
}

pub fn conflicts(existing: &[Booking], candidate: Booking) -> bool {
    existing.iter().any(|b| overlap(*b, candidate))
}

fn overlap(a: Booking, b: Booking) -> bool {
    if a.start > b.start {
        return overlap(b, a);
    }

    match (a.end, b.end) {
        (Some(ae), Some(be)) => b.start <= ae && a.start <= be,
        (Some(ae), None) => b.start <= ae,
        (None, Some(_)) => true,
        (None, None) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_bookings_do_not_conflict() {
        let existing = [Booking::new(10, Some(20))];
        assert!(!conflicts(&existing, Booking::new(20, Some(25))));
        assert!(!conflicts(&existing, Booking::new(5, Some(10))));
    }

    #[test]
    fn partial_overlap_and_containment_conflict() {
        let existing = [Booking::new(10, Some(20))];
        assert!(conflicts(&existing, Booking::new(19, Some(25))));
        assert!(conflicts(&existing, Booking::new(12, Some(18))));
        assert!(conflicts(&existing, Booking::new(5, Some(15))));
    }

    #[test]
    fn open_ended_booking_only_conflicts_when_candidate_reaches_it() {
        let existing = [Booking::new(30, None)];
        assert!(!conflicts(&existing, Booking::new(10, Some(30))));
        assert!(conflicts(&existing, Booking::new(10, Some(31))));
        assert!(conflicts(&existing, Booking::new(35, Some(40))));
    }

    #[test]
    fn candidate_open_ended_conflicts_only_if_it_extends_past_existing_start() {
        let existing = [Booking::new(30, Some(40))];
        assert!(!conflicts(&existing, Booking::new(10, Some(30))));
        assert!(conflicts(&existing, Booking::new(10, None)));
    }

    #[test]
    fn any_existing_conflict_is_detected() {
        let existing = [
            Booking::new(0, Some(5)),
            Booking::new(10, Some(15)),
            Booking::new(20, None),
        ];
        assert!(conflicts(&existing, Booking::new(14, Some(21))));
        assert!(!conflicts(&existing, Booking::new(15, Some(20))));
    }
}
