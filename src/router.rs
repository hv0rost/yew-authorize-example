use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::authorization::AuthForm;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route  {
    #[at("/")]
    Authorization,
}

pub fn switch(route : &Route) -> Html {
    match route {
        Route::Authorization => html!(<AuthForm />),
    }
}