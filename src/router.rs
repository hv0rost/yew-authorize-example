use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::authorization::AuthForm;
use crate::components::users::UserComponent;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route  {
    #[at("/")]
    Authorization,
    #[at("/users")]
    Users,
}

pub fn switch(route : &Route) -> Html {
    match route {
        Route::Authorization => html!(<AuthForm />),
        Route::Users => html!(<UserComponent />),
    }
}