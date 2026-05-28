#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Root,
    Item,
    Search,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Auth {
    Guest,
    User,
    Admin,
}

pub fn action(method: Method, resource: Resource, auth: Auth) -> &'static str {
    match (method, resource, auth) {
        (Method::Get, Resource::Root, _) => "index",
        (Method::Get, Resource::Item, Auth::Guest) => "view_item",
        (Method::Get, Resource::Item, _) => "edit_item",
        (Method::Get, Resource::Search, _) => "search",
        (Method::Post, Resource::Root, Auth::Guest) => "reject",
        (Method::Post, Resource::Root, _) => "create_root",
        (Method::Post, Resource::Item, _) => "update_item",
        (Method::Post, Resource::Search, _) => "search",
        (Method::Delete, Resource::Root, _) => "reject",
        (Method::Delete, Resource::Item, Auth::Admin) => "delete_item",
        (Method::Delete, Resource::Item, _) => "reject",
        (Method::Delete, Resource::Search, _) => "clear_search",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_item_depends_on_auth() {
        assert_eq!(action(Method::Get, Resource::Item, Auth::Guest), "view_item");
        assert_eq!(action(Method::Get, Resource::Item, Auth::User), "view_item");
        assert_eq!(action(Method::Get, Resource::Item, Auth::Admin), "edit_item");
    }

    #[test]
    fn posting_item_requires_non_guest_and_admin_has_same_behavior() {
        assert_eq!(action(Method::Post, Resource::Item, Auth::Guest), "reject");
        assert_eq!(action(Method::Post, Resource::Item, Auth::User), "update_item");
        assert_eq!(action(Method::Post, Resource::Item, Auth::Admin), "update_item");
    }

    #[test]
    fn search_routes_distinguish_read_write_and_delete_is_rejected() {
        assert_eq!(action(Method::Get, Resource::Search, Auth::Guest), "search");
        assert_eq!(action(Method::Post, Resource::Search, Auth::User), "save_search");
        assert_eq!(action(Method::Delete, Resource::Search, Auth::Admin), "reject");
    }

    #[test]
    fn root_rules_stay_as_defined() {
        assert_eq!(action(Method::Get, Resource::Root, Auth::Guest), "index");
        assert_eq!(action(Method::Post, Resource::Root, Auth::Guest), "reject");
        assert_eq!(action(Method::Post, Resource::Root, Auth::Admin), "create_root");
        assert_eq!(action(Method::Delete, Resource::Root, Auth::Admin), "reject");
    }
}
