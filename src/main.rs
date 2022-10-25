mod components;
mod util;
mod router;
mod store;

use router::{Route, switch};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <BrowserRouter>
        <Switch<Route> render = {Switch::render(switch)} />
      </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}


