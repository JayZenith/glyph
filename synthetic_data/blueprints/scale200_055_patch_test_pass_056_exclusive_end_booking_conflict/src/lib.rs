pub fn has_conflict(existing: &[(u32, u32)], start: u32, end: u32) -> bool {
    existing
        .iter()
        .any(|&(s, e)| start <= e && s <= end)
}

#[cfg(test)]
mod tests {
    use super::has_conflict;

    #[test]
    fn overlapping_booking_conflicts() {
        let bookings = vec![(10, 20), (30, 40)];
        assert!(has_conflict(&bookings, 15, 18));
        assert!(has_conflict(&bookings, 18, 22));
    }

    #[test]
    fn touching_endpoints_do_not_conflict() {
        let bookings = vec![(10, 20), (30, 40)];
        assert!(!has_conflict(&bookings, 20, 30));
        assert!(!has_conflict(&bookings, 40, 50));
        assert!(!has_conflict(&bookings, 0, 10));
    }
}
