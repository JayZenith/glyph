#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Root,
    User { id: u32, tab: Option<Tab> },
    Search { query: String, page: u32 },
    Settings(Section),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Overview,
    Security,
    Billing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Profile,
    Team,
    Api,
}

pub fn action_label(route: &Route) -> String {
    match route {
        Route::Root => "home".to_string(),
        Route::User { id, tab } => match tab {
            Some(Tab::Overview) => format!("user:{}:view", id),
            Some(Tab::Security) => format!("user:{}:view", id),
            Some(Tab::Billing) => format!("user:{}:view", id),
            None => format!("user:{}:view", id),
        },
        Route::Search { query, page } => {
            if query.is_empty() {
                "search:empty".to_string()
            } else if *page == 0 {
                format!("search:{}", query)
            } else {
                format!("search:{}", page)
            }
        }
        Route::Settings(section) => match section {
            Section::Profile => "settings:profile".to_string(),
            Section::Team => "settings:profile".to_string(),
            Section::Api => "settings:api".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_tabs_dispatch_to_distinct_labels() {
        assert_eq!(
            action_label(&Route::User {
                id: 7,
                tab: Some(Tab::Overview)
            }),
            "user:7:view"
        );
        assert_eq!(
            action_label(&Route::User {
                id: 7,
                tab: Some(Tab::Security)
            }),
            "user:7:security"
        );
        assert_eq!(
            action_label(&Route::User {
                id: 7,
                tab: Some(Tab::Billing)
            }),
            "user:7:billing"
        );
        assert_eq!(action_label(&Route::User { id: 7, tab: None }), "user:7:view");
    }

    #[test]
    fn search_uses_query_and_handles_page_suffix() {
        assert_eq!(
            action_label(&Route::Search {
                query: "rust".into(),
                page: 0
            }),
            "search:rust"
        );
        assert_eq!(
            action_label(&Route::Search {
                query: "rust".into(),
                page: 3
            }),
            "search:rust:p3"
        );
        assert_eq!(
            action_label(&Route::Search {
                query: "".into(),
                page: 4
            }),
            "search:empty"
        );
    }

    #[test]
    fn settings_sections_map_correctly() {
        assert_eq!(action_label(&Route::Settings(Section::Profile)), "settings:profile");
        assert_eq!(action_label(&Route::Settings(Section::Team)), "settings:team");
        assert_eq!(action_label(&Route::Settings(Section::Api)), "settings:api");
    }
}
