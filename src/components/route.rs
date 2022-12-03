use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::multi::MultiPlay;
use crate::components::single::SinglePlay;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/single")]
    SinglePlay,
    #[at("/multi")]
    MultiPlay,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Redirect<Route> to={Route::SinglePlay}/> },
        Route::SinglePlay => html! { <SinglePlay/> },
        Route::MultiPlay => html! { <MultiPlay/> },
        Route::NotFound => html! { <Redirect<Route> to={Route::Home}/> },
    }
}
