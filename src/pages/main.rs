use crate::router::Route;
use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[function_component(MainPage)]
pub fn main_page() -> Html {
    html! {
        <div class="container w-[200px] m-[20px]">
            <div class="flex flex-col gap-[10px]">
                <h1 class="text-center">{"Tetrust: Main Page" }</h1>

                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded-full">
                    <Link<Route> to={Route::SinglePlay}>{ "Single" }</Link<Route>>
                </button>

                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded-full">
                    <Link<Route> to={Route::MultiPlay}>{ "Multi" }</Link<Route>>
                </button>

                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded-full">
                    <Link<Route> to={Route::Setting}>{ "Setting" }</Link<Route>>
                </button>
            </div>
        </div>
    }
}
