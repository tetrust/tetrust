use crate::components::gamebox;

use yew::{function_component, html};

#[function_component(MainComponent)]
pub fn main_component() -> Html {
    html! {
        <main>
            <gamebox::GameBox/>
        </main>
    }
}
