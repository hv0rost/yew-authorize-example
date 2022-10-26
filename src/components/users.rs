use std::rc::Rc;

use gloo::console::log;
use serde::{Serialize, Deserialize};
use yewdux::prelude::*;
use yew::prelude::*;

use crate::store::store::{UserStore, AuthStore};

pub struct UserComponent {
    dispatch: Dispatch<UserStore>,
    response: Dispatch<AuthStore>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Token {
    pub accesstoken: String,
    pub refreshtoken: String,
    pub code : i32,
    pub message : Option<String>,
}

pub enum Msg {
    UserStore(Rc<UserStore>),
    AuthStore(Rc<AuthStore>),
}

impl Component for UserComponent{
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        log!("created");
        let dispatch = Dispatch::<UserStore>::subscribe(ctx.link().callback(Msg::UserStore));
        let response = Dispatch::<AuthStore>::subscribe(ctx.link().callback(Msg::AuthStore));
           Self { dispatch, response }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let auth_value = self.response.get().response.to_owned().unwrap_or_default();
        let users : Token = serde_json::from_value(auth_value["userLogIn"].clone()).unwrap();
        html! {
            <>
            <div>{users.accesstoken}</div>
            </>
        }
    }
}