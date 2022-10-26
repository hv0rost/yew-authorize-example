use yew::prelude::*;
use yewdux::prelude::*;
use std::rc::Rc;
use gloo::console::log;
use web_sys::HtmlInputElement;
use graphql_client::GraphQLQuery;
use std::fmt::Debug;

use yew_router::prelude::*;
use crate::router::Route;

use crate::{store::store::AuthStore, util::common::fetch_gql_data};

pub enum Msg {
    Store(Rc<AuthStore>),
    Username(String),
    Password(String),
    SendRequest,
    SetState(Option<serde_json::Value>),
}

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user_log_in.graphql",
    response_derives = "Debug",
)]
pub struct UserLogIn;

pub struct  AuthForm {
    dispatch: Dispatch<AuthStore>
}

async fn query_str(email: String, password: String,) -> String {
    let build_query =
    UserLogIn::build_query(user_log_in::Variables { email, password, pincode : None });
    let query = serde_json::json!(build_query);

    query.to_string()
}

impl Component for AuthForm {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<AuthStore>::subscribe(ctx.link().callback(Msg::Store));
        Self { dispatch }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Store(_) => false,
            Msg::Username(username) => {
                self.dispatch.reduce_mut(|store| store.username = Some(username));
                false
            }
            Msg::Password(password) => {
                self.dispatch.reduce_mut(|store| store.password = Some(password));
                false
            },
            Msg::SendRequest => {
                let username = self.dispatch.get().username.as_deref().unwrap_or_default().to_string();
                let password = self.dispatch.get().password.as_deref().unwrap_or_default().to_string();
                ctx.link().send_future(async{
                    match fetch_gql_data(&query_str(username, password).await).await
                    {
                        Ok(data) => {
                            log!(serde_json::to_string_pretty(&data).unwrap());
                            Msg::SetState(Some(data))
                        },
                        Err(err) => {
                            log!(err.to_string());
                            Msg::SetState(Some(serde_json::Value::Null))
                        }
                    }
                });
                false
            }
            Msg::SetState(data) => {
                self.dispatch.reduce_mut(|store| store.response = Some(data.clone().unwrap()));
                if !(data.unwrap().is_null()){
                    let history = ctx.link().history().unwrap();
                    history.push(Route::Users);
                }
                true   
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|e : FocusEvent|{
            e.prevent_default();
            Msg::SendRequest
        });

        let username_onchange = ctx.link().callback(|e : Event|{
            let target = e.target_unchecked_into::<HtmlInputElement>();
            let username = target.value();
            Msg::Username(username)
        });

        let password_onchange = ctx.link().callback(|e : Event|{
            let target = e.target_unchecked_into::<HtmlInputElement>();
            let password = target.value();
            Msg::Password(password)
        });

        html!(
            <>
                <div class="container">
                <div class="screen">
                    <div class="screen__content">
                        <form class="login" {onsubmit} autocomplete ="on">
                            <div class="login__field">
                                <i class="login__icon fas fa-user"></i>
                                <input type="text" class="login__input" placeholder="User name / Email" onchange={username_onchange}/>
                            </div>
                            <div class="login__field">
                                <i class="login__icon fas fa-lock"></i>
                                <input type="password" class="login__input" placeholder="Password" onchange={password_onchange}/>
                            </div>
                            <button class="button login__submit">
                                <span class="button__text">{"Log In Now"}</span>
                                <i class="button__icon fas fa-chevron-right"></i>
                            </button>				
                        </form>
                        <div class="social-login">
                            <h3>{"log in via"}</h3>
                            <div class="social-icons">
                                <a href="#" class="social-login__icon fab fa-instagram"></a>
                                <a href="#" class="social-login__icon fab fa-facebook"></a>
                                <a href="#" class="social-login__icon fab fa-twitter"></a>
                            </div>
                        </div>
                    </div>
                    <div class="screen__background">
                        <span class="screen__background__shape screen__background__shape4"></span>
                        <span class="screen__background__shape screen__background__shape3"></span>		
                        <span class="screen__background__shape screen__background__shape2"></span>
                        <span class="screen__background__shape screen__background__shape1"></span>
                    </div>		
                </div>
            </div>
                </>
        )
    }
}