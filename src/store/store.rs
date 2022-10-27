use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

use super::entities::User;

#[derive(Default, PartialEq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Default, PartialEq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct Token {
    pub accesstoken: String,
}

#[derive(Default, PartialEq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct UserStore{pub user : Vec<User>}
