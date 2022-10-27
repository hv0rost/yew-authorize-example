use yew::prelude::*;
use yewdux::prelude::*;
use std::rc::Rc;
use gloo::console::log;
use web_sys::HtmlInputElement;
use graphql_client::GraphQLQuery;
use std::fmt::Debug;

use yew_router::prelude::*;
use crate::router::Route;

use crate::{store::store::{AuthStore, Token}, util::common::fetch_gql_data};

pub struct  AuthForm {
    dispatch: Dispatch<AuthStore>,
    creditianals: Dispatch<Token>
}

pub enum Msg {
    Store(Rc<AuthStore>),
    Token(Rc<Token>),
    Username(String),
    Password(String),
    SendRequest,
    Success(Option<serde_json::Value>),
    Error(String),
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
        let creditianals = Dispatch::<Token>::subscribe(ctx.link().callback(Msg::Token));

        Self { dispatch, creditianals }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Store(_) => false,
            Msg::Token(_) => false,
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
                    match fetch_gql_data(&query_str(username, password).await, "".to_string()).await
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
            }
            Msg::Success(data) => {
                log!(serde_json::to_string_pretty(&data).unwrap());
                let token : Token = serde_json::from_value(data.clone().unwrap()["userLogIn"].clone()).unwrap();
                self.creditianals.reduce_mut(|store| store.accesstoken = token.accesstoken);
                
                if !(data.unwrap().is_null()){
                    let history = ctx.link().history().unwrap();
                    history.push(Route::Users);
                }
                true   
            },
            Msg::Error(err) => {
                log!(err);
                false
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