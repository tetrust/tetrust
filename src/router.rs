use yew::{function_component, html, Html};
use yew_router::prelude::*;

use crate::pages::{multi::MultiPlay, setting::SettingPage, single::SinglePlay, main::MainPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/single")]
    SinglePlay,
    #[at("/multi")]
    MultiPlay,
    #[at("/setting")]
    Setting,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <MainPage/> },
        Route::SinglePlay => html! { <SinglePlay/> },
        Route::MultiPlay => html! { <MultiPlay/> },
        Route::Setting => {
            html! { <SettingPage/> }
        }
        Route::NotFound => html! { <Redirect<Route> to={Route::Home}/> },
    }
}

#[function_component(MainRouterComponent)]
pub fn main_router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
