use yew::{function_component, html};
use yew_router::prelude::*;

use crate::components::route::{switch, Route};

#[function_component(MainComponent)]
pub fn main_component() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
