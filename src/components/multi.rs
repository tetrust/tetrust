use crate::components::gamebox;

use yew::{function_component, html, Html};

#[function_component(MultiPlay)]
pub fn multiplay() -> Html {
    html! {
        <div>
            <gamebox::GameBox/>
        </div>
    }
}
