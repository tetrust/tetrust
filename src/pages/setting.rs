use crate::lib::components::setting::arr::ArrInput;
use crate::lib::components::setting::das::DasInput;
use yew::{function_component, html, Html};

#[function_component(SettingPage)]
pub fn setting_page() -> Html {
    html! {
        <div>
            <h1>{ "Setting Page" }</h1>

            <div>
                <DasInput/>
            </div>

            <div>
                <ArrInput/>
            </div>
        </div>
    }
}
