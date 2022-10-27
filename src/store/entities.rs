use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Store, Default)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub isadmin: bool,
    pub datecreate: i64,
    pub owner: i32,
    pub clientid: Option<i32>,
    pub params: Option<serde_json::Value>,
    pub accesstoken: Option<String>,
    pub refreshtoken: Option<String>,
    pub disabled: bool,
    pub owner_by_id : Option<Box<User>>,   
}