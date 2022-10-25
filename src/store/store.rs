use yewdux::prelude::*;

#[derive(Default, PartialEq, Store, Clone)]
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
}
