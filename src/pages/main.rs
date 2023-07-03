use crate::lib::components::header::Header;
use crate::router::Route;

use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[function_component(MainPage)]
pub fn main_page() -> Html {
    html! {
        <div>
            <Header/>

            <h1>{ "Main Page" }</h1>

            <div>
                <Link<Route> to={Route::SinglePlay}>{ "Single" }</Link<Route>>
            </div>

            <div>
                <Link<Route> to={Route::MultiPlay}>{ "Multi" }</Link<Route>>
            </div>

            <div>
                <Link<Route> to={Route::Setting}>{ "Setting" }</Link<Route>>
            </div>
        </div>
    }
}
