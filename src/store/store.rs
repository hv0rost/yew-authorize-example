use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Default, PartialEq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
    pub response : Option<serde_json::Value>
}

#[derive(Default, PartialEq, Store, Clone, Serialize, Deserialize)]
pub struct UserStore {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub isadmin: bool,
    pub datecreate: usize,
    pub owner: i32,
    pub clientid: Option<i32>,
    pub params: Option<serde_json::Value>,
    pub accesstoken: Option<String>,
    pub refreshtoken: Option<String>,
    pub disabled: bool,
    pub owner_by_id : Option<Box<UserStore>>,   
}
