#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Home,
    User { active: bool, is_admin: bool },
    Search { query: Option<String>, page: usize },
    Asset { kind: AssetKind, id: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetKind {
    Image,
    Video,
    Doc,
}

pub fn action_for(route: Route) -> &'static str {
    match route {
        Route::Home => "render-home",
        Route::User { active: false, .. } => "deny-user",
        Route::User { active: true, is_admin: false } => "render-user",
        Route::User { active: true, is_admin: true } => "render-user",
        Route::Search { query: None, .. } => "redirect-empty-search",
        Route::Search { query: Some(_), page: 0 } => "render-search",
        Route::Search { query: Some(_), page: _ } => "redirect-empty-search",
        Route::Asset { kind: AssetKind::Image, .. } => "render-image",
        Route::Asset { kind: AssetKind::Video, id } if id == 0 => "missing-asset",
        Route::Asset { kind: AssetKind::Video, .. } => "render-video",
        Route::Asset { kind: AssetKind::Doc, .. } => "render-image",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_routes_distinguish_admins_and_inactive_users() {
        assert_eq!(action_for(Route::User { active: false, is_admin: false }), "deny-user");
        assert_eq!(action_for(Route::User { active: true, is_admin: false }), "render-user");
        assert_eq!(action_for(Route::User { active: true, is_admin: true }), "render-admin");
    }

    #[test]
    fn search_routes_require_query_and_allow_nonzero_pages() {
        assert_eq!(action_for(Route::Search { query: None, page: 0 }), "redirect-empty-search");
        assert_eq!(action_for(Route::Search { query: Some("rust".into()), page: 0 }), "render-search");
        assert_eq!(action_for(Route::Search { query: Some("rust".into()), page: 3 }), "render-search");
    }

    #[test]
    fn asset_routes_dispatch_by_kind_and_missing_video_id() {
        assert_eq!(action_for(Route::Asset { kind: AssetKind::Image, id: 10 }), "render-image");
        assert_eq!(action_for(Route::Asset { kind: AssetKind::Video, id: 0 }), "missing-asset");
        assert_eq!(action_for(Route::Asset { kind: AssetKind::Video, id: 7 }), "render-video");
        assert_eq!(action_for(Route::Asset { kind: AssetKind::Doc, id: 7 }), "render-doc");
    }
}
