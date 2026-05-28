pub fn count_valid_records(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| is_valid_record(line))
        .count()
}

fn is_valid_record(line: &str) -> bool {
    let mut seen_id = false;
    let mut seen_name = false;
    let mut seen_age = false;

    for field in line.split(';') {
        let Some((key, value)) = field.split_once('=') else {
            return false;
        };

        match key.trim() {
            "id" => {
                if value.trim().parse::<u32>().is_err() {
                    return false;
                }
                seen_id = true;
            }
            "name" => {
                if value.trim().is_empty() {
                    return false;
                }
                seen_name = true;
            }
            "age" => {
                if value.trim().parse::<u8>().is_err() {
                    return false;
                }
                seen_age = true;
            }
            _ => return false,
        }
    }

    seen_id || seen_name || seen_age
}

#[cfg(test)]
mod tests {
    use super::count_valid_records;

    #[test]
    fn counts_only_records_with_all_required_fields() {
        let input = "id=1;name=Ana;age=30\nid=2;name=Bob\nname=Cat;age=7\nid=3;name=Dee;age=9\n";
        assert_eq!(count_valid_records(input), 2);
    }

    #[test]
    fn rejects_unknown_fields_and_bad_numbers() {
        let input = "id=1;name=Ana;age=30;city=Oslo\nid=x;name=Bob;age=8\nid=4;name=Eve;age=255\n";
        assert_eq!(count_valid_records(input), 0);
    }

    #[test]
    fn accepts_spaces_around_keys_and_values_but_not_empty_name() {
        let input = " id = 5 ; name = Fay ; age = 41 \nid = 6; name =   ; age = 22\n";
        assert_eq!(count_valid_records(input), 1);
    }
}
