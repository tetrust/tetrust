use crate::lib::components::setting::arr::ArrInput;
use crate::lib::components::setting::das::DasInput;
use crate::lib::components::setting::sdf::SdfInput;
use yew::{function_component, html, Html};

#[function_component(MainPage)]
pub fn main_page() -> Html {
    html! {
        <div>
            <h1>{ "Main Page" }</h1>

            <div>
                <DasInput/>
            </div>

            <div>
                <ArrInput/>
            </div>

            <div>
                <SdfInput/>
            </div>
        </div>
    }
}
