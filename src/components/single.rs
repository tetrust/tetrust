use crate::components::gamebox;

use yew::{function_component, html};

#[function_component(SinglePlay)]
pub fn single() -> Html {
    html! {
        <div>
            <gamebox::GameBox/>
        </div>
    }
}
