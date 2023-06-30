use yew_router::prelude::*;

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

use yew::{function_component, html, Html};

use super::{multi::MultiPlay, setting::SinglePlay};

#[function_component(MainRouterComponent)]
pub fn main_router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
