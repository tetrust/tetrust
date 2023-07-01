use crate::components::game_ui;

use yew::{function_component, html, Html};

#[function_component(SinglePlay)]
pub fn single() -> Html {
    html! {
        <div>
            <game_ui::GameUI/>
        </div>
    }
}
