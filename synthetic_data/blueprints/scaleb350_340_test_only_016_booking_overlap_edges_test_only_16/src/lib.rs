pub fn conflicts(existing: &[(u32, u32)], request: (u32, u32)) -> bool {
    let (start, end) = request;
    assert!(start < end, "request interval must be non-empty");

    existing.iter().any(|&(a, b)| {
        assert!(a < b, "existing interval must be non-empty");
        start < b && a < end
    })
}

#[cfg(test)]
mod tests {
    use super::conflicts;

    #[test]
    fn overlap_in_middle_conflicts() {
        let booked = vec![(10, 20), (30, 40)];
        assert!(conflicts(&booked, (15, 18)));
        assert!(conflicts(&booked, (18, 35)));
    }

    #[test]
    fn touching_endpoint_does_not_conflict() {
        let booked = vec![(10, 20), (30, 40)];
        assert!(!conflicts(&booked, (20, 30)));
        assert!(!conflicts(&booked, (0, 10)));
        assert!(!conflicts(&booked, (40, 50)));
    }

    #[test]
    fn contained_or_covering_range_conflicts() {
        let booked = vec![(10, 20)];
        assert!(conflicts(&booked, (12, 19)));
        assert!(conflicts(&booked, (5, 25)));
    }

    #[test]
    fn empty_schedule_never_conflicts() {
        let booked: Vec<(u32, u32)> = vec![];
        assert!(!conflicts(&booked, (3, 8)));
    }
}
