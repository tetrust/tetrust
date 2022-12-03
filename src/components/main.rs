use yew::{function_component, html, Html};
use yew_router::prelude::*;

use crate::components::route::{switch, Route};

#[function_component(MainComponent)]
pub fn main_component() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
