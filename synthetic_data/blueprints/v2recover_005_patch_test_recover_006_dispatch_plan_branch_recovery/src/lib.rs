#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Create,
    Update,
    Delete,
    Sync,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub mode: Mode,
    pub urgent: bool,
    pub dry_run: bool,
    pub id: Option<u32>,
    pub payload: Option<&'static str>,
}

pub fn build_plan(req: &Request) -> Result<Vec<&'static str>, &'static str> {
    if req.dry_run && matches!(req.mode, Mode::Delete) {
        return Err("delete dry-run unsupported");
    }

    let mut steps = vec![];

    match req.mode {
        Mode::Create => {
            if req.id.is_some() {
                return Err("create should not include id");
            }
            if req.payload.is_none() {
                return Err("create needs payload");
            }
            steps.push("validate_payload");
            if req.urgent {
                steps.push("priority_queue");
            }
            steps.push("insert_record");
        }
        Mode::Update => {
            if req.id.is_none() {
                return Err("update needs id");
            }
            steps.push("load_current");
            if req.payload.is_some() {
                steps.push("apply_patch");
            }
            steps.push("store_record");
        }
        Mode::Delete => {
            if req.id.is_none() {
                return Err("delete needs id");
            }
            steps.push("archive_record");
            steps.push("remove_record");
        }
        Mode::Sync => {
            if req.payload.is_some() {
                return Err("sync ignores payload");
            }
            if req.urgent {
                steps.push("priority_queue");
            }
            steps.push("fetch_remote");
            steps.push("merge_state");
        }
    }

    if req.dry_run {
        steps.push("commit");
    }

    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_urgent_builds_validation_insert_and_simulate() {
        let req = Request {
            mode: Mode::Create,
            urgent: true,
            dry_run: true,
            id: None,
            payload: Some("doc"),
        };

        assert_eq!(
            build_plan(&req).unwrap(),
            vec!["validate_payload", "priority_queue", "insert_record", "simulate"]
        );
    }

    #[test]
    fn update_without_payload_still_requires_targeted_load_and_store() {
        let req = Request {
            mode: Mode::Update,
            urgent: false,
            dry_run: false,
            id: Some(7),
            payload: None,
        };

        assert_eq!(
            build_plan(&req).unwrap(),
            vec!["load_current", "store_record"]
        );
    }

    #[test]
    fn delete_with_dry_run_archives_and_simulates() {
        let req = Request {
            mode: Mode::Delete,
            urgent: false,
            dry_run: true,
            id: Some(3),
            payload: None,
        };

        assert_eq!(
            build_plan(&req).unwrap(),
            vec!["archive_record", "remove_record", "simulate"]
        );
    }

    #[test]
    fn sync_requires_id_and_ignores_urgent_priority() {
        let missing_id = Request {
            mode: Mode::Sync,
            urgent: false,
            dry_run: false,
            id: None,
            payload: None,
        };
        assert_eq!(build_plan(&missing_id), Err("sync needs id"));

        let with_id = Request {
            mode: Mode::Sync,
            urgent: true,
            dry_run: false,
            id: Some(9),
            payload: None,
        };
        assert_eq!(
            build_plan(&with_id).unwrap(),
            vec!["fetch_remote", "merge_state"]
        );
    }
}
