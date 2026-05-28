#[derive(Debug, Clone, Copy)]
pub struct Record {
    pub id: u32,
    pub active: bool,
}

pub fn active_even_ids(records: &[Record]) -> Vec<u32> {
    records
        .iter()
        .filter(|r| r.active)
        .map(|r| r.id)
        .filter(|id| id % 2 == 1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_only_active_even_ids_in_order() {
        let records = [
            Record { id: 1, active: true },
            Record { id: 2, active: true },
            Record { id: 3, active: false },
            Record { id: 4, active: true },
            Record { id: 6, active: false },
            Record { id: 8, active: true },
        ];

        assert_eq!(active_even_ids(&records), vec![2, 4, 8]);
    }

    #[test]
    fn returns_empty_when_no_active_even_ids_exist() {
        let records = [
            Record { id: 1, active: true },
            Record { id: 2, active: false },
            Record { id: 5, active: true },
        ];

        assert!(active_even_ids(&records).is_empty());
    }
}
