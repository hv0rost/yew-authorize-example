use std::rc::Rc;
use gloo::console::log;
use yewdux::prelude::*;
use yew::prelude::*;
use graphql_client::GraphQLQuery;
use std::fmt::Debug;

use crate::{store::store::{UserStore, Token}, util::common::fetch_gql_data};
use crate::store::entities::User;

pub struct UserComponent {
    dispatch: Dispatch<UserStore>,
    token: Dispatch<Token>,
}
pub enum Msg {
    UserStore(Rc<UserStore>),
    Token(Rc<Token>),
    Success(Option<serde_json::Value>),
    Error(String),
}

type Date = i64;
type JSON = serde_json::Value;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_users.graphql",
    response_derives = "Debug",
)]
pub struct AllUsers;

async fn query_str() -> String {
    let build_query =
    AllUsers::build_query(all_users::Variables);
    let query = serde_json::json!(build_query);

    query.to_string()
}

impl Component for UserComponent{
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<UserStore>::subscribe(ctx.link().callback(Msg::UserStore));
        let token = Dispatch::<Token>::subscribe(ctx.link().callback(Msg::Token));

           Self { dispatch, token }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UserStore(_) => false,
            Msg::Token(_) => {
                let token = self.token.get().accesstoken.to_string();
                ctx.link().send_future(async {
                    match fetch_gql_data(&query_str().await, token).await
                    {
                        Ok(data) => {
                            Msg::Success(Some(data))
                        },
                        Err(err) => {
                            Msg::Error(err.to_string())
                        }
                    }
                });
                false
            },
            Msg::Success(data) => {
                let users  = serde_json::json!(data.clone().unwrap()["allUsers"].clone()).as_array().unwrap().to_owned();
                let mut user_vec : Vec<User> = Vec::new();
                
                for user in users {
                    user_vec.push(serde_json::from_value::<User>(user).unwrap_or_default()); 
                }

                log!(serde_json::to_string_pretty(&user_vec).unwrap());
                self.dispatch.reduce(|_| UserStore{
                    user: user_vec,
                });

                true
            },
            Msg::Error(err) => {
                log!(err.to_string());
                false
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let users = self.dispatch.get().user.clone();
        html! {
            <div id="introductions">
        {
            users.into_iter().map(|user| {
                html!{
                    <>
                        <h4 key={user.id.clone()}>{ format!("ID : {}",user.id.clone()) }</h4>
                        <h4 key={user.email.clone()}>{ format!("Email : {}",user.email) }</h4>
                        <h4 key={user.name.clone().unwrap()}>{ format!("Name : {}",user.name.unwrap()) }</h4>
                    </>
                }
            }).collect::<Html>()
        }
    </div>
        }
    }
}