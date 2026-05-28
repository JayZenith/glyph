pub fn conflicts(existing: &[(u32, u32)], candidate: (u32, u32)) -> bool {
    let (start, end) = candidate;
    if start >= end {
        return true;
    }

    existing
        .iter()
        .any(|&(s, e)| start <= e && s <= end)
}

#[cfg(test)]
mod tests {
    use super::conflicts;

    #[test]
    fn overlapping_intervals_conflict() {
        let slots = vec![(10, 20), (30, 40)];
        assert!(conflicts(&slots, (15, 18)));
        assert!(conflicts(&slots, (18, 35)));
    }

    #[test]
    fn adjacent_intervals_do_not_conflict() {
        let slots = vec![(10, 20), (30, 40)];
        assert!(!conflicts(&slots, (20, 30)));
        assert!(!conflicts(&slots, (40, 50)));
        assert!(!conflicts(&slots, (0, 10)));
    }

    #[test]
    fn invalid_candidate_conflicts() {
        let slots = vec![(10, 20)];
        assert!(conflicts(&slots, (5, 5)));
        assert!(conflicts(&slots, (9, 2)));
    }
}
